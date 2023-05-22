#[cfg(test)]
mod tests {
    use serialize::{Decode, Encode};

    #[test]
    fn p1_hex_to_base64() {
        let initial = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(initial.bytes_from_hex().unwrap().to_base64(), expected);
    }
}
