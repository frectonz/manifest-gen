use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct Icon {
    src: String,
    sizes: String,
}

impl Icon {
    pub fn set_src(&mut self, src: String) {
        self.src = src;
    }

    pub fn set_sizes(&mut self, sizes: String) {
        self.sizes = sizes;
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
