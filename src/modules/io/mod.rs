use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::dataset::Dataset;
use crate::dataelem::{attribute_by_tag, DataElement, DataElementValue, DicomVr};

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

    let ts_uid = String::new();

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
        off += h.len as usize;
    }

    let ts = if ts_uid.is_empty() {
        // Fallback if missing: Implicit Little
        ts_from_uid("1.2.840.10008.1.2")
    } else {
        ts_from_uid(&ts_uid)
    };

    Some((ts, off))
}

pub fn read_dicom<P: AsRef<Path>>(path: P) -> Dataset {
    // Read whole file (fine for small tests; stream for large)
    let mut buffer = Vec::new();
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Dataset::new(),
    };
    if file.read_to_end(&mut buffer).is_err() { return Dataset::new(); }
    if buffer.len() < 132 { return Dataset::new(); }

    // Check Part 10 preamble
    let preamble = &buffer[128..132];
    if preamble != b"DICM" {
        // You could allow raw datasets by starting at 0 and assuming a TS,
        // but this function expects Part 10.
        return Dataset::new();
    }

    // Parse File Meta (Explicit Little)
    let (ts, mut off) = match parse_file_meta(&buffer, 132) {
        Some(v) => v,
        None => return Dataset::new(),
    };

    // Iterate dataset
    let mut ds = Dataset::new();
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
            return ds;
        }
        let val = &buffer[off..off + (hdr.len as usize)];
        off += hdr.len as usize;

        // Build dataset entries
        let tag_str = format!("({:04X},{:04X})", hdr.group, hdr.element);
        if hdr.group == 0x7FE0 && hdr.element == 0x0010 {
            // Pixel Data: keep attribute entry without duplicating bytes
            ds.set_pixel_data(val.to_vec());
            if let Some(attr) = attribute_by_tag(&tag_str) {
                ds.push(DataElement { attribute: attr, value: None });
            }
            continue;
        }
        if let Some(attr) = attribute_by_tag(&tag_str) {
            let parsed_value = match attr.vr {
                Some(DicomVr::Ae) | Some(DicomVr::As) | Some(DicomVr::Cs) | Some(DicomVr::Da)
                | Some(DicomVr::Ds) | Some(DicomVr::Dt) | Some(DicomVr::Is) | Some(DicomVr::Lo)
                | Some(DicomVr::Lt) | Some(DicomVr::Pn) | Some(DicomVr::Sh) | Some(DicomVr::St)
                | Some(DicomVr::Tm) | Some(DicomVr::Uc) | Some(DicomVr::Ui) | Some(DicomVr::Ur)
                | Some(DicomVr::Ut) => {
                    let s = std::str::from_utf8(val).unwrap_or("");
                    let s = s.trim_end_matches(['\0', ' ']);
                    Some(DataElementValue::String(s.to_string()))
                }
                Some(DicomVr::Us) => {
                    if val.len() == 2 { Some(DataElementValue::UInt16(match ts.endian { Endianness::Little => u16::from_le_bytes([val[0], val[1]]), Endianness::Big => u16::from_be_bytes([val[0], val[1]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Ss) => {
                    if val.len() == 2 { Some(DataElementValue::Int16(match ts.endian { Endianness::Little => i16::from_le_bytes([val[0], val[1]]), Endianness::Big => i16::from_be_bytes([val[0], val[1]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Ul) => {
                    if val.len() == 4 { Some(DataElementValue::UInt32(match ts.endian { Endianness::Little => u32::from_le_bytes([val[0], val[1], val[2], val[3]]), Endianness::Big => u32::from_be_bytes([val[0], val[1], val[2], val[3]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Sl) => {
                    if val.len() == 4 { Some(DataElementValue::Int32(match ts.endian { Endianness::Little => i32::from_le_bytes([val[0], val[1], val[2], val[3]]), Endianness::Big => i32::from_be_bytes([val[0], val[1], val[2], val[3]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Uv) => {
                    if val.len() == 8 { Some(DataElementValue::UInt64(match ts.endian { Endianness::Little => u64::from_le_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]), Endianness::Big => u64::from_be_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Sv) => {
                    if val.len() == 8 { Some(DataElementValue::Int64(match ts.endian { Endianness::Little => i64::from_le_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]), Endianness::Big => i64::from_be_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Fd) => {
                    if val.len() == 8 { Some(DataElementValue::Double(match ts.endian { Endianness::Little => f64::from_le_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]), Endianness::Big => f64::from_be_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::Fl) => {
                    if val.len() == 4 { Some(DataElementValue::Float(match ts.endian { Endianness::Little => f32::from_le_bytes([val[0], val[1], val[2], val[3]]), Endianness::Big => f32::from_be_bytes([val[0], val[1], val[2], val[3]]) })) } else { Some(DataElementValue::Data(val.to_vec())) }
                }
                Some(DicomVr::At) => {
                    if val.len() == 4 {
                        let g = match ts.endian { Endianness::Little => u16::from_le_bytes([val[0], val[1]]), Endianness::Big => u16::from_be_bytes([val[0], val[1]]) };
                        let e = match ts.endian { Endianness::Little => u16::from_le_bytes([val[2], val[3]]), Endianness::Big => u16::from_be_bytes([val[2], val[3]]) };
                        Some(DataElementValue::Tag(g, e))
                    } else {
                        Some(DataElementValue::Data(val.to_vec()))
                    }
                }
                // Binary or complex VRs: keep raw
                _ => Some(DataElementValue::Data(val.to_vec())),
            };
            ds.push(DataElement { attribute: attr, value: parsed_value });
        }
    }

    ds
}
