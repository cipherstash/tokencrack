use sha2::digest::generic_array::GenericArray;
use sha2::digest::OutputSizeUser;
use sha2::Digest;

pub type Token<T> = GenericArray<u8, <T as OutputSizeUser>::OutputSize>;

#[inline]
pub fn tokenize<T>(input: &[u8]) -> Token<T> where T: Digest + OutputSizeUser {
    T::new()
        .chain_update(input)
        .finalize()
}

#[inline]
pub fn tokenize_with_salt<T>(input: &str, salt: &[u8]) -> Token<T> where T: Digest + OutputSizeUser {
    T::new()
        .chain_update(input.as_bytes())
        .chain_update(salt)
        .finalize()
}

