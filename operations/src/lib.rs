pub trait Xor {
    fn xor(&self, other: &Self) -> Vec<u8>;
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
}
