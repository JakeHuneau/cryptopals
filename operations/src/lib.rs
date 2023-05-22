pub trait Xor {
    fn xor_with(&mut self, other: &Self);
}

impl Xor for Vec<u8> {
    fn xor_with(&mut self, other: &Self) {
        for (byte, other_byte) in self.iter_mut().zip(other) {
            *byte ^= *other_byte;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_with() {
        let mut initial: Vec<u8> = vec![1, 2, 3]; // [01, 10, 11]
        let other: Vec<u8> = vec![2, 3, 1]; // [10, 11, 01]
        initial.xor_with(&other);
        let expected: Vec<u8> = vec![3, 1, 2]; // [11, 01, 10]
        assert_eq!(initial, expected);
    }
}
