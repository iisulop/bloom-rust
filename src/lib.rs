//! Bloom filter implementation for efficient probabilistic checking if a key exists in a dataset.

use log::debug;
use bit_vec::BitVec;

/// Trait for the general bloom filter operations
pub trait BloomFilterOperation {
    fn insert(&mut self, key: &[u8]);
    fn contains_key(&self, key: &[u8]) -> bool;
}

type HashFunction = Vec<fn(&[u8]) -> u128>;

/// Bloom filter struct containing the bitfield and hash functions related to it
pub struct BloomFilter {
    bitfield: BitVec,
    hash_functions: HashFunction,
}

/// Bloom filter implementation specific functions
impl BloomFilter {
    /// Initialize a bloom filter
    pub fn new(bit_array_size: usize, hash_functions: HashFunction) -> BloomFilter {
        BloomFilter {
            bitfield: BitVec::from_elem(bit_array_size, false),
            hash_functions,
        }
    }
}

/// Operations available for the bloom filters
impl crate::BloomFilterOperation for BloomFilter {
    /// Insert key into the bloom filter
    fn insert(&mut self, key: &[u8]) {
        debug!("Inserting key");
        let bf_len = self.bitfield.len() as u128;
        for f in self.hash_functions.iter() {
            let h = f(key);
            let pos = h % bf_len;
            self.bitfield.set(pos as usize, true);
        }
        debug!("{:#?}", self.bitfield);
    }

    /// Check if the bloom filter contains the given key
    fn contains_key(&self, key: &[u8]) -> bool {
        let bf_len = self.bitfield.len() as u128;
        for f in self.hash_functions.iter() {
            let h = f(key);
            let pos = h % bf_len;
            if !self.bitfield[pos as usize] {
                return false
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use fasthash::murmur3;
    use rand::Rng;

    fn ean_code() -> Vec<u8> {
         let num = rand::thread_rng().gen_range(100_000_000_000u64, 999_999_999_999u64).to_string();
         let mut checknum: u8 = 0;
         let mut multiplier: u8 = 3;
         if num.len() % 2 == 1 {
            multiplier = 1
         }
         for n in num.chars() {
            checknum += (multiplier * n.to_digit(10).unwrap() as u8) % 10;
         }
         let res = format!("{}{}", num, checknum);
         res.into_bytes()
    }

    fn hash_function(input: &[u8]) -> u128 {
        murmur3::hash128(&input)
    }

    #[test]
    fn init() {
        crate::BloomFilter::new(10, vec![hash_function]);
    }

    #[test]
    fn add_key() {
        use crate::BloomFilterOperation;
        let mut bf = crate::BloomFilter::new(10, vec![hash_function]);
        let key = b"key";
        bf.insert(key);
    }

    #[test]
    fn contains_key() {
        use crate::BloomFilterOperation;
        let mut bf = crate::BloomFilter::new(10, vec![hash_function]);
        let key = ean_code();
        let result = bf.contains_key(&key);
        assert_eq!(result, false);
        bf.insert(&key);
        let result = bf.contains_key(&key);
        assert_eq!(result, true);
    }
}

