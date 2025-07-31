use std::fs::File;
use std::io::{Read};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Endianness {
    Little,
    Big,
}

#[derive(Clone, Copy, Debug)]
enum VrMode {
    Explicit,
    Implicit,
}

#[derive(Clone, Copy, Debug)]
struct TransferSyntax {
    endian: Endianness,
    vr_mode: VrMode,
}

fn ts_from_uid(uid: &str) -> TransferSyntax {
    match uid {
        // Implicit VR Little Endian
        "1.2.840.10008.1.2" => TransferSyntax {
            endian: Endianness::Little,
            vr_mode: VrMode::Implicit,
        },
        // Explicit VR Little Endian
        "1.2.840.10008.1.2.1" => TransferSyntax {
            endian: Endianness::Little,
            vr_mode: VrMode::Explicit,
        },
        // Explicit VR Big Endian (rare)
        "1.2.840.10008.1.2.2" => TransferSyntax {
            endian: Endianness::Big,
            vr_mode: VrMode::Explicit,
        },
        // Encapsulated transfer syntaxes are still Explicit Little for tags
        _ => TransferSyntax {
            endian: Endianness::Little,
            vr_mode: VrMode::Explicit,
        },
    }
}

fn read_u16(buf: &[u8], off: &mut usize, e: Endianness) -> Option<u16> {
    if *off + 2 > buf.len() {
        return None;
    }
    let v = match e {
        Endianness::Little => u16::from_le_bytes([buf[*off], buf[*off + 1]]),
        Endianness::Big => u16::from_be_bytes([buf[*off], buf[*off + 1]]),
    };
    *off += 2;
    Some(v)
}

fn read_u32(buf: &[u8], off: &mut usize, e: Endianness) -> Option<u32> {
    if *off + 4 > buf.len() {
        return None;
    }
    let v = match e {
        Endianness::Little => u32::from_le_bytes([
            buf[*off],
            buf[*off + 1],
            buf[*off + 2],
            buf[*off + 3],
        ]),
        Endianness::Big => u32::from_be_bytes([
            buf[*off],
            buf[*off + 1],
            buf[*off + 2],
            buf[*off + 3],
        ]),
    };
    *off += 4;
    Some(v)
}

#[derive(Debug)]
struct ElemHeader {
    group: u16,
    element: u16,
    vr: Option<[u8; 2]>,
    len: u32,
}

fn read_elem_header(
    buf: &[u8],
    off: &mut usize,
    endian: Endianness,
    vr_mode: VrMode,
) -> Option<ElemHeader> {
    let group = read_u16(buf, off, endian)?;
    let element = read_u16(buf, off, endian)?;
    match vr_mode {
        VrMode::Explicit => {
            if *off + 2 > buf.len() {
                return None;
            }
            let vr = [buf[*off], buf[*off + 1]];
            *off += 2;
            let is_long = matches!(&vr, b"OB" | b"OW" | b"OF" | b"SQ" | b"UT" | b"UN");
            if is_long {
                // skip 2 reserved bytes
                if *off + 2 > buf.len() {
                    return None;
                }
                *off += 2;
                let len = read_u32(buf, off, endian)?;
                Some(ElemHeader {
                    group,
                    element,
                    vr: Some(vr),
                    len,
                })
            } else {
                if *off + 2 > buf.len() {
                    return None;
                }
                let l = match endian {
                    Endianness::Little => {
                        u16::from_le_bytes([buf[*off], buf[*off + 1]]) as u32
                    }
                    Endianness::Big => {
                        u16::from_be_bytes([buf[*off], buf[*off + 1]]) as u32
                    }
                };
                *off += 2;
                Some(ElemHeader {
                    group,
                    element,
                    vr: Some(vr),
                    len: l,
                })
            }
        }
        VrMode::Implicit => {
            let len = read_u32(buf, off, endian)?;
            Some(ElemHeader {
                group,
                element,
                vr: None,
                len,
            })
        }
    }
}

// Parse File Meta (group 0002) in Explicit Little Endian starting at off.
// Returns (transfer_syntax, new_offset)
fn parse_file_meta(buf: &[u8], mut off: usize) -> Option<(TransferSyntax, usize)> {
    // File Meta starts immediately after "DICM"
    // It must be Explicit Little regardless of dataset TS
    let endian = Endianness::Little;
    let vr_mode = VrMode::Explicit;

    let mut ts_uid = String::new();

    // Optional: read (0002,0000) to know how far to go. But we can
    // loop until we encounter a tag with group != 0x0002.
    loop {
        let save = off;
        let h: ElemHeader = read_elem_header(buf, &mut off, endian, vr_mode)?;
        if h.group != 0x0002 {
            // rewind to start of this element; it's part of the main dataset
            off = save;
            break;
        }
        if off + (h.len as usize) > buf.len() {
            return None;
        }
        let val = &buf[off..off + (h.len as usize)];
        off += h.len as usize;

        if h.element == 0x0010 {
            // TransferSyntaxUID
            let s = std::str::from_utf8(val).ok()?;
            ts_uid = s.trim_end_matches(|c| c == '\0' || c == ' ').to_string();
            let vr_disp = match h.vr {
                Some(v) => String::from_utf8_lossy(&v).to_string(),
                None => "UN".to_string(), // Implicit VR has no VR
            };
            println!(
                "({:04X},{:04X}) VR={} len={} TransferSyntaxUID: {}", 
                h.group,
                h.element,
                vr_disp,
                ts_uid.len(),
                ts_uid
            );
        } else {
            // Other File Meta elements can be ignored for now
            println!(
                "({:04X},{:04X}) VR={} len={} value={}",
                h.group,
                h.element,
                h.vr.map_or("UN".to_string(), |v| String::from_utf8_lossy(&v).to_string()),
                h.len,
                std::str::from_utf8(val).unwrap_or("")
            );
        }
    }

    let ts = if ts_uid.is_empty() {
        // Fallback if missing: Implicit Little
        ts_from_uid("1.2.840.10008.1.2")
    } else {
        ts_from_uid(&ts_uid)
    };

    Some((ts, off))
}

pub fn read_dicom<P: AsRef<Path>>(path: P) -> bool {
    // Read whole file (fine for small tests; stream for large)
    let mut buffer = Vec::new();
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    if file.read_to_end(&mut buffer).is_err() {
        return false;
    }
    if buffer.len() < 132 {
        return false;
    }

    // Check Part 10 preamble
    let preamble = &buffer[128..132];
    if preamble != b"DICM" {
        // You could allow raw datasets by starting at 0 and assuming a TS,
        // but this function expects Part 10.
        return false;
    }

    // Parse File Meta (Explicit Little)
    let (ts, mut off) = match parse_file_meta(&buffer, 132) {
        Some(v) => v,
        None => return false,
    };

    // Iterate dataset
    loop {
        if off + 8 > buffer.len() {
            break;
        }
        let hdr = match read_elem_header(&buffer, &mut off, ts.endian, ts.vr_mode) {
            Some(h) => h,
            None => break,
        };

        // Undefined length: 0xFFFFFFFF. Usually for SQ/OB/OW.
        if hdr.len == 0xFFFF_FFFF {
            eprintln!(
                "Encountered undefined length at ({:04X},{:04X}); stop for now",
                hdr.group, hdr.element
            );
            break;
        }

        // Bounds check value
        if off + (hdr.len as usize) > buffer.len() {
            eprintln!("Truncated value at ({:04X},{:04X})", hdr.group, hdr.element);
            return false;
        }
        let val = &buffer[off..off + (hdr.len as usize)];
        off += hdr.len as usize;

        // Print samples
        if hdr.group == 0x7FE0 && hdr.element == 0x0010 {
            println!("({:04X},{:04X}) Pixel Data: {} bytes len={}", hdr.group, hdr.element, val.len(), hdr.len);
        } else {
            let vr_disp = match hdr.vr {
                Some(v) => String::from_utf8_lossy(&v).to_string(),
                None => "UN".to_string(), // Implicit VR has no VR
            };
            let s = std::str::from_utf8(&val[0..hdr.len as usize]).unwrap_or("");
            println!(
                "({:04X},{:04X}) VR={} len={} value={}",
                hdr.group, hdr.element, vr_disp, hdr.len, s
            );
        }
    }

    true
}