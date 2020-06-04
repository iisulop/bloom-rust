#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn hash_function(_input: &[u8]) -> &[u8] {
        &[255, 0]
    }

    #[test]
    fn init() {
        crate::bloom::Bloom::new(10, vec!(hash_function));
    }
}

pub mod bloom {
    type HashFunction = Vec<fn(&[u8]) -> &[u8]>;

    pub struct Bloom {
        bitfield: Vec<u8>,
        hash_functions: HashFunction,
    }

    impl Bloom {
        pub fn new(bit_array_size: usize, hash_functions: HashFunction) -> Bloom {
            Bloom {
                bitfield: Vec::with_capacity(bit_array_size),
                hash_functions
            }
        }

        pub fn calculate_bit_array_size(
            // bit_array_size: u64,
            num_elements: f64,
            // num_hash_functions: f64,
            num_false_positives: f64,
        ) -> f64 {
            let e = 1.0_f64.exp();
            (-num_elements*e.log(num_false_positives))/(e.log(2.0_f64).powf(2.0_f64))
        }
    }
}
