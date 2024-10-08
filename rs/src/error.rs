#[derive(Debug, Clone)]
pub struct SecretSantaError {
    pub msg: String,
}

impl SecretSantaError {
    pub fn new(msg: String) -> SecretSantaError {
        return SecretSantaError { msg };
    }
}
