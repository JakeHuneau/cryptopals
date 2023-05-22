#[cfg(test)]
mod tests {
    use operations::Xor;
    use serialize::{Decode, Encode};
    use std::collections::HashMap;

    #[test]
    fn ch1_hex_to_base64() {
        let initial = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(initial.bytes_from_hex().unwrap().to_base64(), expected);
    }

    #[test]
    fn ch2_fixed_xor() {
        let initial = String::from("1c0111001f010100061a024b53535009181c")
            .bytes_from_hex()
            .unwrap();
        let other = String::from("686974207468652062756c6c277320657965")
            .bytes_from_hex()
            .unwrap();
        let result = initial.xor(&other);
        assert_eq!(
            result.to_hex(),
            String::from("746865206b696420646f6e277420706c6179")
        );
    }

    #[test]
    fn ch3_single_byte_xor() {
        let initial =
            String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .bytes_from_hex()
                .unwrap();
        let weights = HashMap::from([
            (' ', 0.3132),
            ('A', 0.3132),
            ('B', 0.2163),
            ('C', 0.3906),
            ('D', 0.3151),
            ('E', 0.2673),
            ('F', 0.1416),
            ('G', 0.1876),
            ('H', 0.2321),
            ('I', 0.3211),
            ('J', 0.1726),
            ('K', 0.0687),
            ('L', 0.1884),
            ('M', 0.3529),
            ('N', 0.2085),
            ('O', 0.1842),
            ('P', 0.2614),
            ('Q', 0.0316),
            ('R', 0.2519),
            ('S', 0.4003),
            ('T', 0.3322),
            ('U', 0.0814),
            ('V', 0.0892),
            ('W', 0.2527),
            ('X', 0.0343),
            ('Y', 0.0304),
            ('Z', 0.0076),
            ('a', 5.1880),
            ('b', 1.0195),
            ('c', 2.1129),
            ('d', 2.5071),
            ('e', 8.5771),
            ('f', 1.3725),
            ('g', 1.5597),
            ('h', 2.7444),
            ('i', 4.9019),
            ('j', 0.0867),
            ('k', 0.6753),
            ('l', 3.1750),
            ('m', 1.6437),
            ('n', 4.9701),
            ('o', 5.7701),
            ('p', 1.5482),
            ('q', 0.0747),
            ('r', 4.2586),
            ('s', 4.3686),
            ('t', 6.3700),
            ('u', 2.0999),
            ('v', 0.8462),
            ('w', 1.3034),
            ('x', 0.1950),
            ('y', 1.1330),
            ('z', 0.0596),
        ]);
        let mut decoding = String::from("");
        let mut top_score: f64 = 0.;
        (0u8..=255).for_each(|enc_byte| {
            let xor: Vec<u8> = initial.xor(&vec![enc_byte]);
            if xor.iter().all(|&xor_byte| xor_byte > 31 && xor_byte <= 123) {
                // Make sure it's a valid ASCII
                println!("{enc_byte}");
                let score = xor
                    .iter()
                    .map(|&byte| {
                        let upper = byte as char;
                        println!("{upper}");
                        match weights.get(&(byte as char)) {
                            Some(&value) => value,
                            None => 0.,
                        }
                    })
                    .sum::<f64>();
                println!("Score: {score}");
                if score > top_score {
                    top_score = score;
                    decoding = xor.into_iter().map(|ch| ch as char).collect::<String>();
                }
            }
        });
        assert_eq!(decoding, String::from("Cooking MC's like a pound of bacon"));
    }
}
