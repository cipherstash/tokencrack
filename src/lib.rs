pub mod generators;
pub mod token;
use generators::Generator;
use sha2::digest::OutputSizeUser;
use sha2::Digest;
pub use sha2::{Sha256, Sha512};
use std::marker::PhantomData;
use token::{tokenize, Token};

pub struct Cracker<G, D>
where
    D: OutputSizeUser + Digest,
    G: Generator,
{
    _target: PhantomData<D>,
    generator: G,
}

impl<G, D> Cracker<G, D>
where
    D: OutputSizeUser + Digest,
    G: Generator,
{
    pub fn new(generator: G) -> Self {
        Self {
            _target: PhantomData,
            generator,
        }
    }
}

impl<G, D> Cracker<G, D>
where
    D: OutputSizeUser + Digest,
    G: Generator,
    String: From<<G as Generator>::Item>,
{
    pub fn crack(&self, target: &[u8]) -> Option<String> {
        let target = Token::<D>::clone_from_slice(target);
        self.generator
            .generate()
            .map(String::from)
            .find(|line| tokenize::<D>(line.as_ref()) == target)
    }

    pub fn crack_with_salt(&self, target: &[u8], _salt: &[u8]) -> Option<String> {
        let _target = Token::<D>::clone_from_slice(target);
        //self.generator.generate().find(|line| tokenize_with_salt::<D>(line, salt) == target).map(String::from)
        unimplemented!()
    }
}
