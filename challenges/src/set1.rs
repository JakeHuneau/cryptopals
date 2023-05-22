#[cfg(test)]
mod tests {
    use operations::Xor;
    use serialize::{Decode, Encode};

    #[test]
    fn ch1_hex_to_base64() {
        let initial = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(initial.bytes_from_hex().unwrap().to_base64(), expected);
    }

    #[test]
    fn ch2_fixed_xor() {
        let mut initial = String::from("1c0111001f010100061a024b53535009181c")
            .bytes_from_hex()
            .unwrap();
        let other = String::from("686974207468652062756c6c277320657965")
            .bytes_from_hex()
            .unwrap();
        initial.xor_with(&other);
        assert_eq!(
            initial.to_hex(),
            String::from("746865206b696420646f6e277420706c6179")
        );
    }
}
