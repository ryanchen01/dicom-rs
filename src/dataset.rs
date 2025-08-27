use crate::dataelem::*;

#[derive(Debug, Default, Clone)]
pub struct Dataset {
    file_meta: Vec<DataElement>,
    data_elements: Vec<DataElement>,
    pixel_data: Option<Vec<u8>>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            file_meta: Vec::new(),
            data_elements: Vec::new(),
            pixel_data: None,
        }
    }

    pub fn set_file_meta(&mut self, meta: Vec<DataElement>) {
        self.file_meta = meta;
    }

    pub fn file_meta(&self) -> &[DataElement] {
        &self.file_meta
    }

    pub fn push(&mut self, elem: DataElement) {
        self.data_elements.push(elem);
    }

    pub fn set_pixel_data(&mut self, data: Vec<u8>) {
        self.pixel_data = Some(data);
    }

    pub fn elements(&self) -> &[DataElement] {
        &self.data_elements
    }

    pub fn pixel_data(&self) -> Option<&[u8]> {
        self.pixel_data.as_deref()
    }

    pub fn get(&self, tag_or_keyword: &str) -> Option<&DataElement> {
        // Try interpret input as tag first, then as keyword
        if let Some(attr) = attribute_by_tag(tag_or_keyword) {
            let tag = attr.tag;
            // Search File Meta first, then main dataset
            if let Some(found) = self.file_meta.iter().find(|de| de.attribute.tag == tag) {
                return Some(found);
            }
            if let Some(found) = self.data_elements.iter().find(|de| de.attribute.tag == tag) {
                return Some(found);
            }
            return None;
        }
        if let Some(attr) = attribute_by_keyword(tag_or_keyword) {
            let tag = attr.tag;
            if let Some(found) = self.file_meta.iter().find(|de| de.attribute.tag == tag) {
                return Some(found);
            }
            if let Some(found) = self.data_elements.iter().find(|de| de.attribute.tag == tag) {
                return Some(found);
            }
            return None;
        }
        None
    }
}
