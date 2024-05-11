mod shift;

pub use shift::Shift;

pub trait Encrypt<C> {
    type Output;
    fn encrypt_with(&self, cipher: &C) -> Self::Output;
}
