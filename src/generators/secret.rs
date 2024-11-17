use super::Generator;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
const MAX_CHARSET: u8 = CHARSET.len() as u8;

pub struct SecretIterator<const N: usize> {
    idx: usize,
    counters: [u8; N],
    full: bool,
}

impl<const N: usize> SecretIterator<N> {
    pub fn new() -> Self {
        Self {
            counters: [0; N],
            idx: 0,
            full: false,
        }
    }

    fn increment_counters(&mut self) -> bool {
        if increment_counters(&mut self.counters, self.idx) {
            self.idx += 1;
            println!("Incrementing idx to {}", self.idx);
            if self.idx == N {
                return true;
            }
        }
        false
    }
}

// Increments and returns true if the counter is at the maximum value
fn increment_counters(counter: &mut [u8], idx: usize) -> bool {
    counter[idx] += 1;
    if counter[idx] == MAX_CHARSET {
        counter[idx] = 0;
        if idx == 0 {
            return true;
        } else {
            return increment_counters(counter, idx - 1);
        }
    }
    false
}

impl<const N: usize> Iterator for SecretIterator<N> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.full {
            return None;
        }
        let s = build_string(self.counters, self.idx + 1);
        if self.increment_counters() {
            self.full = true;
        }
        Some(s)
    }
}

fn build_string<const N: usize>(counters: [u8; N], length: usize) -> String {
    let mut s = String::new();
    for i in 0..length {
        let b = counters[i];
        s.push(CHARSET[b as usize] as char);
    }
    s
}

pub struct Secret<const N: usize>;

impl<const N: usize> Generator for Secret<N> {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(SecretIterator::<N>::new())
    }
}
