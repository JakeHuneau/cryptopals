pub trait Decode {
    fn bytes_from_hex(&self) -> Vec<u8>;
    fn bytes_from_base64(&self) -> Vec<u8>;
}

pub trait Encode {
    fn to_base64(&self) -> String;
    fn to_hex(&self) -> String;
}

impl Decode for String {
    fn bytes_from_hex(&self) -> Vec<u8> {
        (0..self.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&self[i..i + 2], 16).expect("Could not convert bytes to hex")
            })
            .collect()
    }

    fn bytes_from_base64(&self) -> Vec<u8> {
        let mut n = self.len();
        if self.as_bytes()[n - 1] == b'=' {
            if self.as_bytes()[n - 2] == b'=' {
                n -= 1;
            }
            n -= 1;
        }

        let mut nums = Vec::with_capacity(n);
        for c in self.chars().take(n) {
            nums.push(u8_from_base64(c));
        }

        let mut result = Vec::with_capacity(3 * n / 4);
        for b in nums.chunks(4) {
            result.push((b[0] << 2) + (b[1] >> 4));
            if b.len() == 2 {
                if b[1] << 4 != 0 {
                    panic!("UHOH");
                }
                break;
            }

            result.push((b[1] << 4) + (b[2] >> 2));
            if b.len() == 3 {
                if b[2] << 6 != 0 {
                    panic!("input not padded")
                }
                break;
            }

            result.push((b[2] << 6) + b[3]);
        }
        result
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

    fn to_hex(&self) -> String {
        self.iter()
            .map(|b| format!("{b:02x?}"))
            .collect::<Vec<String>>()
            .join("")
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

fn u8_from_base64(value: char) -> u8 {
    match value {
        'A'..='Z' => value as u8 - b'A',
        'a'..='z' => 26 + (value as u8 - b'a'),
        '0'..='9' => 52 + (value as u8 - b'0'),
        '+' => 62,
        '/' => 63,
        _ => panic!("base64 could not be convered to u8"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_from_hex() {
        let initial = String::from("00ff45");
        let expected: Vec<u8> = vec![0, 255, 69];
        assert_eq!(initial.bytes_from_hex(), expected);
    }

    #[test]
    fn test_bytes_from_base64() {
        let initial = String::from("SSdt");
        let initial_2 = String::from("SSdt=");
        let initial_3 = String::from("SSdt==");
        let expected = vec![73, 39, 109];
        assert_eq!(initial.bytes_from_base64(), expected);
        assert_eq!(initial_2.bytes_from_base64(), expected);
        assert_eq!(initial_3.bytes_from_base64(), expected);
    }

    #[test]
    fn test_bytes_to_base64() {
        let initial: Vec<u8> = vec![73, 39, 109];
        assert_eq!(initial.to_base64(), String::from("SSdt"))
    }

    #[test]
    fn test_bytes_to_hex() {
        let initial: Vec<u8> = vec![0, 255, 69];
        let expected = String::from("00ff45");
        assert_eq!(initial.to_hex(), expected);
    }
}
