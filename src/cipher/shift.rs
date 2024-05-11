use crate::Encrypt;
use modicum::*;

const ALPHABET_SIZE: usize = 26;

pub struct Shift {
    shift: i8,
}

impl Shift {
    pub fn new(shift: i8) -> Self {
        Self { shift }
    }
}

impl Encrypt<Shift> for char {
    type Output = char;
    fn encrypt_with(&self, cipher: &Shift) -> Self::Output {
        if self.is_ascii_alphabetic() {
            let base = if self.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = *self as i8 - base as i8;
            let shifted = offset.add_mod(cipher.shift, ALPHABET_SIZE);
            (base + shifted as u8) as char
        } else {
            *self
        }
    }
}

impl Encrypt<Shift> for String {
    type Output = String;
    fn encrypt_with(&self, cipher: &Shift) -> Self::Output {
        self.chars().map(|c| c.encrypt_with(cipher)).collect()
    }
}

impl Encrypt<Shift> for &str {
    type Output = String;
    fn encrypt_with(&self, cipher: &Shift) -> String {
        self.to_string().encrypt_with(cipher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_positive_shift() {
        let plaintext = "Hello, World!";
        let cipher = Shift::new(3);
        let ciphertext = plaintext.encrypt_with(&cipher);
        assert_eq!(ciphertext, "Khoor, Zruog!");
    }

    #[test]
    fn encrypt_negative_shift() {
        let plaintext = "Hello, World!";
        let cipher = Shift::new(-3);
        let ciphertext = plaintext.encrypt_with(&cipher);
        assert_eq!(ciphertext, "Ebiil, Tloia!");
    }
}
