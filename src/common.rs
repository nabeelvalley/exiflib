pub struct Jpeg {
    pub bytes: Vec<u8>,
}

pub struct ImageFile {
    pub format: String,
    pub version: String,
    pub model: String,
    pub identifier: String,
    pub jpeg: Jpeg,
}
