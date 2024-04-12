mod caesar;

pub use caesar::Caesar;

pub trait Encrypt<C> {
    type Output;
    fn encrypt_with(&self, cipher: C) -> Self::Output;
}
