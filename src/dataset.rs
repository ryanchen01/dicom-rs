use crate::dataelem::*;

#[derive(Debug, Default, Clone)]
pub struct Dataset {
    data_elements: Vec<DataElement>,
    pixel_data: Option<Vec<u8>>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            data_elements: Vec::new(),
            pixel_data: None,
        }
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
        if let Some(attr) = attribute_by_tag(tag_or_keyword) {
            let tag = attr.tag;
            return self
                .data_elements
                .iter()
                .find(|de| de.attribute.tag == tag);
        }
        if let Some(attr) = attribute_by_keyword(tag_or_keyword) {
            let tag = attr.tag;
            return self
                .data_elements
                .iter()
                .find(|de| de.attribute.tag == tag);
        }
        None
    }
}
