const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"; //abcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
const MAX_CHARSET: u8 = CHARSET.len() as u8 - 1;

pub struct Secret<const N: usize> {
    idx: usize,
    current_size: usize,
    counters: [u8; N],
}

impl<const N: usize> Secret<N> {
    pub fn new() -> Self {
        Self {
            counters: [0; N],
            current_size: 1,
            idx: 0,
        }
    }

    fn increment_counters(&mut self) {
        if self.counters[self.idx] == MAX_CHARSET {
            self.counters[self.idx] = 0;
            self.idx += 1;
        }

        if self.idx == self.current_size {
            self.current_size += 1;
            self.idx = 0;
            self.counters = [0; N];
        }

        self.counters[self.idx] += 1;
    }
}

impl<const N: usize> Iterator for Secret<N> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counters == [MAX_CHARSET; N] {
            return None;
        }

        let s = build_string(self.counters, self.current_size);
        self.increment_counters();
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

#[cfg(test)]
mod test {
    #[test]
    fn test_iterate() {
        let secret = super::Secret::<2>::new();
        for x in secret {
            println!("{}", x);
        }
    }
}
