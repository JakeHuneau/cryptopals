use std::num::ParseIntError;

pub trait Decode {
    fn bytes_from_hex(&self) -> Result<Vec<u8>, ParseIntError>;
}

pub trait Encode {
    fn to_base64(&self) -> String;
}

impl Decode for &str {
    fn bytes_from_hex(&self) -> Result<Vec<u8>, ParseIntError> {
        (0..self.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self[i..i + 2], 16))
            .collect()
    }
}

impl Encode for Vec<u8> {
    fn to_base64(&self) -> String {
        let mut cycle: u8 = 0;
        let mut base64 = String::from("");
        let mut index: usize = 0;
        while index < self.len() {
            match cycle {
                0 => {
                    base64.push(u8_to_base64(self.get(index).unwrap() >> 2));
                }
                1 => {
                    let first_value = (self.get(index).unwrap() & 3) << 4;
                    let next_value = self.get(index + 1).unwrap() >> 4;
                    let new_value = first_value | next_value;
                    base64.push(u8_to_base64(new_value));
                    index += 1;
                }
                2 => {
                    let first_value = (self.get(index).unwrap() & 15) << 2;
                    let next_value = self.get(index + 1).unwrap() >> 6;
                    let new_value = first_value | next_value;
                    base64.push(u8_to_base64(new_value));
                    index += 1;
                }
                3 => {
                    base64.push(u8_to_base64(self.get(index).unwrap() & 63));
                    index += 1;
                }
                _ => panic!("Cycle out of range!"),
            };
            cycle += 1;
            cycle %= 4;
        }
        base64
    }
}

fn u8_to_base64(value: u8) -> char {
    match value {
        0..=25 => (b'A' + value) as char,
        26..=51 => (b'a' + value - 26) as char,
        52..=61 => (b'0' + value - 52) as char,
        62 => '+',
        63 => '/',
        _ => panic!("u8 string could not be converted to base64"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_from_hex() {
        let initial = "00ff45";
        let expected: Vec<u8> = vec![0, 255, 69];
        assert_eq!(initial.bytes_from_hex().unwrap(), expected);
    }

    #[test]
    fn test_bytes_to_base64() {
        let initial: Vec<u8> = vec![73, 39, 109];
        assert_eq!(initial.to_base64(), String::from("SSdt"))
    }
}
