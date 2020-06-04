#[cfg(test)]
mod tests {
    use fasthash::murmur3;

    fn hash_function(input: &[u8]) -> u128 {
        let h = murmur3::hash128(&input);
        h
    }

    #[test]
    fn init() {
        crate::bloom::BloomFilter::new(10, vec![hash_function]);
    }

    #[test]
    fn add_key() {
        use crate::BloomFilterOperation;
        let mut bf = crate::bloom::BloomFilter::new(10, vec![hash_function]);
        let key = b"key";
        bf.insert(key);
    }

    #[test]
    fn contains_key() {
        use crate::BloomFilterOperation;
        let mut bf = crate::bloom::BloomFilter::new(10, vec![hash_function]);
        let key = b"key";
        let result = bf.contains_key(key);
        assert_eq!(result, false);
        bf.insert(key);
        let result = bf.contains_key(key);
        assert_eq!(result, true);
    }
}

pub trait BloomFilterOperation {
    fn insert(&mut self, key: &[u8]) -> ();
    fn contains_key(&self, key: &[u8]) -> bool;
}

pub mod bloom {
    use bit_vec::BitVec;
    type HashFunction = Vec<fn(&[u8]) -> u128>;

    pub struct BloomFilter {
        bitfield: BitVec,
        hash_functions: HashFunction,
    }

    impl BloomFilter {
        pub fn new(bit_array_size: usize, hash_functions: HashFunction) -> BloomFilter {
            BloomFilter {
                bitfield: BitVec::from_elem(bit_array_size, false),
                hash_functions,
            }
        }

        pub fn calculate_bit_array_size(
            // bit_array_size: u64,
            num_elements: f64,
            // num_hash_functions: f64,
            num_false_positives: f64,
        ) -> f64 {
            let e = 1.0_f64.exp();
            (-num_elements * e.log(num_false_positives)) / (e.log(2.0_f64).powf(2.0_f64))
        }
    }

    impl crate::BloomFilterOperation for BloomFilter {
        fn insert(&mut self, key: &[u8]) {
            let bf_len = self.bitfield.len() as u128;
            for f in self.hash_functions.iter() {
                let h = f(key);
                let pos = h % bf_len;
                self.bitfield.set(pos as usize, true);
            }
        }

        fn contains_key(&self, key: &[u8]) -> bool {
            let bf_len = self.bitfield.len() as u128;
            for f in self.hash_functions.iter() {
                let h = f(key);
                let pos = h % bf_len;
                if !self.bitfield[pos as usize] {
                    return false;
                }
            }
            return true;
        }
    }
}
