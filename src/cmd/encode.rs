use serde::{self, Serialize};
use crate::error::Result;
use crate::aes::encrypt;
use crate::json::JsonPrettyFormatter;

pub fn encode(message: &str, password: &str) -> Result<()> {
    let input_data: String = ["password", password].join("\\u0002");
    let secret_password = hex::encode(input_data.as_bytes());

    let encrypted_data = encrypt(message.as_bytes(), &secret_password.as_ref(), secret_password[0..16].as_bytes()).ok()?;
    let obj = json!({
    				"message": message,
					"encrypted data": hex::encode(&encrypted_data),
					});

    let buf = Vec::new();
    let formatter = JsonPrettyFormatter::new();
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)?;
    println!("{}", String::from_utf8(ser.into_inner())?);
    Ok(())
}