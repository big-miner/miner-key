use crypto::{buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crate::error::Result;

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(final_result)
}


pub fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(final_result)
}


#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use std::str;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn encrypt_decrypt() {
        let message = "QmZtmD2qt6fJot32nabSP3CUjicnypEBz7bHVDhPQt9aAy";
        let password = "46d8f0ad6cc9998ca40c215f07241f994a1a66da5d2b8e2c38f2a36c9e750941";

        let encrypted_data = encrypt(message.as_bytes(), &password.as_ref(), password[0..16].as_bytes()).ok().unwrap();
        let hex_en_data = hex::encode(&encrypted_data);
        let hex_de_data = hex::decode(&hex_en_data).unwrap();
        let decrypted_data = decrypt(&hex_de_data[..], &password.as_ref(), password[0..16].as_bytes()).ok().unwrap();
        assert_eq!(message, str::from_utf8(&decrypted_data[..]).unwrap())
    }
}

