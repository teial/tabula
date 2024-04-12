use crate::Encrypt;

pub struct Caesar {
    shift: i8,
}

impl Caesar {
    pub fn new(shift: i8) -> Self {
        Self { shift }
    }
}

impl Encrypt<Caesar> for String {
    type Output = String;
    fn encrypt_with(&self, cipher: Caesar) -> Self::Output {
        let shift = cipher.shift;
        self.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let index = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = c as i8 - index as i8;
                    let shifted = (offset + shift) % 26;
                    (index + shifted as u8) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Encrypt<Caesar> for &str {
    type Output = String;
    fn encrypt_with(&self, cipher: Caesar) -> String {
        self.to_string().encrypt_with(cipher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn encrypt_positive_shift() {
        let plaintext = "Hello, World!";
        let cipher = Caesar::new(3);
        let ciphertext = plaintext.encrypt_with(cipher);
        assert_eq!(ciphertext, "Khoor, Zruog!");
    }

    #[test]
    fn encrypt_negative_shift() {
        let plaintext = "Hello, World!";
        let cipher = Caesar::new(-3);
        let ciphertext = plaintext.encrypt_with(cipher);
        assert_eq!(ciphertext, "Ebiil, Tloia!");
    }
}
