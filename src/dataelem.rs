use crate::dicts::*;
use std::fmt;

/// DICOM dictionary data structures and lookups generated from parsed/full.csv
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DicomVr {
    Ae,
    As,
    At,
    Cs,
    Da,
    Ds,
    Dt,
    Fd,
    Fl,
    Is,
    Lo,
    Lt,
    Ob,
    ObOrOw,
    Od,
    Of,
    Ol,
    Ov,
    Ow,
    Pn,
    Sh,
    Sl,
    Sq,
    Ss,
    St,
    Sv,
    Tm,
    Uc,
    Ui,
    Ul,
    Un,
    Ur,
    Us,
    UsOrOw,
    UsOrSs,
    Ut,
    Uv,
}

impl DicomVr {
    pub const fn as_str(&self) -> &'static str {
        match self {
            DicomVr::Ae => "AE",
            DicomVr::As => "AS",
            DicomVr::At => "AT",
            DicomVr::Cs => "CS",
            DicomVr::Da => "DA",
            DicomVr::Ds => "DS",
            DicomVr::Dt => "DT",
            DicomVr::Fd => "FD",
            DicomVr::Fl => "FL",
            DicomVr::Is => "IS",
            DicomVr::Lo => "LO",
            DicomVr::Lt => "LT",
            DicomVr::Ob => "OB",
            DicomVr::ObOrOw => "OB or OW",
            DicomVr::Od => "OD",
            DicomVr::Of => "OF",
            DicomVr::Ol => "OL",
            DicomVr::Ov => "OV",
            DicomVr::Ow => "OW",
            DicomVr::Pn => "PN",
            DicomVr::Sh => "SH",
            DicomVr::Sl => "SL",
            DicomVr::Sq => "SQ",
            DicomVr::Ss => "SS",
            DicomVr::St => "ST",
            DicomVr::Sv => "SV",
            DicomVr::Tm => "TM",
            DicomVr::Uc => "UC",
            DicomVr::Ui => "UI",
            DicomVr::Ul => "UL",
            DicomVr::Un => "UN",
            DicomVr::Ur => "UR",
            DicomVr::Us => "US",
            DicomVr::UsOrOw => "US or OW",
            DicomVr::UsOrSs => "US or SS",
            DicomVr::Ut => "UT",
            DicomVr::Uv => "UV",
        }
    }

    pub fn suggested_value_kind(&self) -> ValueKind {
        match self {
            DicomVr::Ae => ValueKind::String,
            DicomVr::As => ValueKind::String,
            DicomVr::At => ValueKind::Tag,
            DicomVr::Cs => ValueKind::String,
            DicomVr::Da => ValueKind::String,
            DicomVr::Ds => ValueKind::String,
            DicomVr::Dt => ValueKind::String,
            DicomVr::Fd => ValueKind::Double,
            DicomVr::Fl => ValueKind::Float,
            DicomVr::Is => ValueKind::String,
            DicomVr::Lo => ValueKind::String,
            DicomVr::Lt => ValueKind::String,
            DicomVr::Ob => ValueKind::Data,
            DicomVr::ObOrOw => ValueKind::String,
            DicomVr::Od => ValueKind::Data,
            DicomVr::Of => ValueKind::Data,
            DicomVr::Ol => ValueKind::Data,
            DicomVr::Ov => ValueKind::Data,
            DicomVr::Ow => ValueKind::Data,
            DicomVr::Pn => ValueKind::String,
            DicomVr::Sh => ValueKind::String,
            DicomVr::Sl => ValueKind::Int32,
            DicomVr::Sq => ValueKind::Sequence,
            DicomVr::Ss => ValueKind::Int16,
            DicomVr::St => ValueKind::String,
            DicomVr::Sv => ValueKind::Int64,
            DicomVr::Tm => ValueKind::String,
            DicomVr::Uc => ValueKind::String,
            DicomVr::Ui => ValueKind::String,
            DicomVr::Ul => ValueKind::UInt32,
            DicomVr::Un => ValueKind::Data,
            DicomVr::Ur => ValueKind::String,
            DicomVr::Us => ValueKind::UInt16,
            DicomVr::UsOrOw => ValueKind::String,
            DicomVr::UsOrSs => ValueKind::String,
            DicomVr::Ut => ValueKind::String,
            DicomVr::Uv => ValueKind::UInt64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueKind { Sequence, String, Data, Int16, Int32, Int64, UInt16, UInt32, UInt64, Float, Double, Tag }

#[derive(Debug, Clone)]
pub enum DataElementValue {
    Sequence(Vec<DataElement>),
    String(String),
    Data(Vec<u8>),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float(f32),
    Double(f64),
    Tag(u16, u16),
}

#[derive(Debug, Clone, Copy)]
pub struct DicomAttribute {
    pub tag: &'static str,
    pub name: &'static str,
    pub keyword: &'static str,
    pub vr: Option<DicomVr>,
    pub vm: &'static str,
    pub attr_type: &'static str,
}

#[derive(Debug, Clone)]
pub struct DataElement {
    pub attribute: &'static DicomAttribute,
    pub value: Option<DataElementValue>,
}

impl fmt::Display for DataElementValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataElementValue::Sequence(items) => write!(f, "SQ[items={}]", items.len()),
            DataElementValue::String(s) => write!(f, "{s}"),
            DataElementValue::Data(b) => write!(f, "<binary {} bytes>", b.len()),
            DataElementValue::Int16(v) => write!(f, "{v}"),
            DataElementValue::Int32(v) => write!(f, "{v}"),
            DataElementValue::Int64(v) => write!(f, "{v}"),
            DataElementValue::UInt16(v) => write!(f, "{v}"),
            DataElementValue::UInt32(v) => write!(f, "{v}"),
            DataElementValue::UInt64(v) => write!(f, "{v}"),
            DataElementValue::Float(v) => write!(f, "{v}"),
            DataElementValue::Double(v) => write!(f, "{v}"),
            DataElementValue::Tag(g, e) => write!(f, "({g:04X},{e:04X})"),
        }
    }
}

impl fmt::Display for DataElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = match &self.value {
            Some(v) => v.to_string(),
            None => String::from("<empty>"),
        };
        write!(f, "{} {} = {}", self.attribute.tag, self.attribute.keyword, value_str)
    }
}

fn normalize_tag(id_or_tag: &str) -> String {
    let s = id_or_tag.trim();
    if s.starts_with('(') && s.ends_with(')') && s.contains(',') {
        return s.to_string();
    }
    let hex: String = s.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    if hex.len() == 8 {
        let group = &hex[..4];
        let element = &hex[4..];
        return format!("({group},{element})");
    }
    s.to_string()
}

pub fn attribute_by_tag(id_or_tag: &str) -> Option<&'static DicomAttribute> {
    let normalized = normalize_tag(id_or_tag);
    if let Ok(idx) = TAG_INDEX.binary_search_by(|entry| entry.0.cmp(normalized.as_str())) {
        let (_, attr_idx) = TAG_INDEX[idx];
        return Some(&ATTRIBUTES[attr_idx]);
    }
    // Fallback linear scan for cases where TAG_INDEX might not be perfectly sorted
    for attr in ATTRIBUTES.iter() {
        if attr.tag == normalized {
            return Some(attr);
        }
    }
    None
}

pub fn attribute_by_keyword(keyword: &str) -> Option<&'static DicomAttribute> {
    if let Ok(idx) = KEYWORD_INDEX.binary_search_by(|entry| entry.0.cmp(keyword)) {
        let (_, attr_idx) = KEYWORD_INDEX[idx];
        return Some(&ATTRIBUTES[attr_idx]);
    }
    // Fallback linear scan for cases where KEYWORD_INDEX might not be perfectly sorted
    for attr in ATTRIBUTES.iter() {
        if attr.keyword == keyword {
            return Some(attr);
        }
    }
    None
}
