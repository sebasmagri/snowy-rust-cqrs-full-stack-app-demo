#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Serialization/Deserialization WASM Error: {0}")]
    SerDe(#[from] serde_wasm_bindgen::Error),
    #[error("HTTP Error: {0}")]
    Http(#[from] reqwest::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
