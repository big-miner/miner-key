use serde::{self, Serialize};
use std::str;
use crate::aes::decrypt;
use crate::error::Result;
use crate::json::JsonPrettyFormatter;

pub fn decode(message: &str, password: &str) -> Result<()> {
    let input_data: String = ["password", password].join("\\u0002");
    let secret_password = hex::encode(input_data.as_bytes());

    let hex_de_message = hex::decode(message)?;
    let decrypted_data = decrypt(&hex_de_message[..], &secret_password.as_ref(), secret_password[0..16].as_bytes()).ok()?;
    let obj = json!({
    				"message": message,
					"Decrypted data": str::from_utf8(&decrypted_data[..])?,
				});

    let buf = Vec::new();
    let formatter = JsonPrettyFormatter::new();
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)?;
    println!("{}", String::from_utf8(ser.into_inner())?);
    Ok(())
}