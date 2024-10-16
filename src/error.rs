use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct SecretSantaError {
    pub msg: String,
}

impl SecretSantaError {
    pub fn new(msg: String) -> SecretSantaError {
        return SecretSantaError { msg };
    }
}

impl Into<JsValue> for SecretSantaError {
    fn into(self) -> JsValue {
        JsValue::from(self.msg)
    }
}
