pub trait Xor {
    fn xor(&self, other: &Self) -> Vec<u8>;
}

pub trait HammingDistance {
    fn hamming_distance(&self, other: &Self) -> usize;
}

pub trait Transpose {
    fn transpose(&self, block_size: usize) -> Vec<Self>
    where
        Self: Sized;
}

impl Xor for Vec<u8> {
    fn xor(&self, other: &Self) -> Vec<u8> {
        let mut result = self.to_vec();
        for chunk in result.chunks_mut(other.len()) {
            for (byte, other_byte) in chunk.iter_mut().zip(other) {
                *byte ^= *other_byte;
            }
        }
        result
    }
}

impl HammingDistance for String {
    fn hamming_distance(&self, other: &Self) -> usize {
        if self.len() != other.len() {
            panic!("Strings must be same length");
        }

        self.bytes()
            .zip(other.bytes())
            .map(|(c1, c2)| {
                let result = c1 ^ c2;
                let mut same_bits: usize = 0;
                for shift in 0..8 {
                    if (result >> shift) & 1 == 1 {
                        same_bits += 1;
                    }
                }
                same_bits
            })
            .sum::<usize>()
    }
}

impl HammingDistance for Vec<u8> {
    fn hamming_distance(&self, other: &Self) -> usize {
        if self.len() != other.len() {
            panic!("Hamming distance requires same length comparisons");
        }

        self.iter()
            .zip(other.iter())
            .map(|(c1, c2)| {
                let result = c1 ^ c2;
                let mut same_bits: usize = 0;
                for shift in 0..8 {
                    if (result >> shift) & 1 == 1 {
                        same_bits += 1;
                    }
                }
                same_bits
            })
            .sum::<usize>()
    }
}

impl HammingDistance for [u8] {
    fn hamming_distance(&self, other: &Self) -> usize {
        if self.len() != other.len() {
            panic!("Hamming distance requires same length comparisons");
        }

        self.iter()
            .zip(other.iter())
            .map(|(c1, c2)| {
                let result = c1 ^ c2;
                let mut same_bits: usize = 0;
                for shift in 0..8 {
                    if (result >> shift) & 1 == 1 {
                        same_bits += 1;
                    }
                }
                same_bits
            })
            .sum::<usize>()
    }
}

impl Transpose for Vec<u8> {
    fn transpose(&self, block_size: usize) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = vec![Vec::with_capacity(self.len()); block_size];
        let chunks = self.chunks(block_size);
        for chunk in chunks {
            for i in 0..block_size {
                if let Some(&byte) = chunk.get(i) {
                    result[i].push(byte);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_with_equal_length() {
        let initial: Vec<u8> = vec![1, 2, 3]; // [01, 10, 11]
        let other: Vec<u8> = vec![2, 3, 1]; // [10, 11, 01]
        let result = initial.xor(&other);
        let expected: Vec<u8> = vec![3, 1, 2]; // [11, 01, 10]
        assert_eq!(result, expected);
    }

    #[test]
    fn xor_with_single_byte() {
        let initial: Vec<u8> = vec![1, 2, 3]; // [01, 10, 11]
        let xor_byte: Vec<u8> = vec![2]; // [10]
        let result = initial.xor(&xor_byte);
        let expected: Vec<u8> = vec![3, 0, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn hamming_distance() {
        let s1 = String::from("this is a test");
        let s2 = String::from("wokka wokka!!!");
        assert_eq!(s1.hamming_distance(&s2), 37);
    }

    #[test]
    fn transpose() {
        let initial: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected: Vec<Vec<u8>> = vec![vec![1, 4, 7, 10], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(initial.transpose(3), expected);
    }
}
