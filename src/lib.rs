mod cipher;

pub use cipher::Encrypt;
pub use cipher::*;

pub fn encrypt<C>(plaintext: &str, cipher: C) -> String
where
    for<'a> &'a str: Encrypt<C, Output = String>,
{
    plaintext.encrypt_with(cipher)
}
