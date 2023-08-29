use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug)]
pub struct Icon {
    src: String,
    sizes: String,
}

impl Icon {
    pub fn new(src: String, sizes: String) -> Self {
        Self { src, sizes }
    }
}

impl Serialize for Icon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Icon", 3)?;
        state.serialize_field("src", &self.src)?;
        state.serialize_field("sizes", &self.sizes)?;
        state.serialize_field("type", "image/png")?;
        state.end()
    }
}
