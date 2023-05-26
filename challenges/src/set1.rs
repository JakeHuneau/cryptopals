use operations::{HammingDistance, Xor};
use std::collections::HashMap;

pub struct DecryptionScore {
    score: f64,
    decrypted_string: String,
    key: u8,
}

pub fn find_best_decryption(encrypted_bytes: &Vec<u8>) -> DecryptionScore {
    let weights = HashMap::from([
        (b' ', 0.167564443682168),
        (b'e', 0.08610229517681191),
        (b't', 0.0632964962389326),
        (b'a', 0.0612553996079051),
        (b'n', 0.05503703643138501),
        (b'i', 0.05480626188138746),
        (b'o', 0.0541904405334676),
        (b's', 0.0518864979648296),
        (b'r', 0.051525029341199825),
        (b'l', 0.03218192615049607),
        (b'd', 0.03188948073064199),
        (b'h', 0.02619237267611581),
        (b'c', 0.02500268898936656),
        (b'u', 0.019247776378510318),
        (b'm', 0.018140172626462205),
        (b'p', 0.017362092874808832),
        (b'f', 0.015750347191785568),
        (b'g', 0.012804659959943725),
        (b'.', 0.011055184780313847),
        (b'y', 0.010893686962847832),
        (b'b', 0.01034644514338097),
        (b'w', 0.009565830104169261),
        (b',', 0.008634492219614468),
        (b'v', 0.007819143740853554),
        (b'0', 0.005918945715880591),
        (b'k', 0.004945712204424292),
        (b'1', 0.004937789430804492),
        (b'S', 0.0030896915651553373),
        (b'T', 0.0030701064687671904),
        (b'C', 0.002987392712176473),
        (b'2', 0.002756237869045172),
        (b'8', 0.002552781042488694),
        (b'5', 0.0025269211093936652),
        (b'A', 0.0024774830020061096),
        (b'9', 0.002442242504945237),
        (b'x', 0.0023064144740073764),
        (b'3', 0.0021865587546870337),
        (b'I', 0.0020910417959267183),
        (b'-', 0.002076717421222119),
        (b'6', 0.0019199098857390264),
        (b'4', 0.0018385271551164353),
        (b'7', 0.0018243295447897528),
        (b'M', 0.0018134911904778657),
        (b'B', 0.0017387002075069484),
        (b'"', 0.0015754276887500987),
        (b'P', 0.00138908405321239),
        (b'E', 0.0012938206232079082),
        (b'N', 0.0012758834637326799),
        (b'F', 0.001220297284016159),
        (b'R', 0.0011037374385216535),
        (b'D', 0.0010927723198318497),
        (b'U', 0.0010426370083657518),
        (b'q', 0.00100853739070613),
        (b'L', 0.0010044809306127922),
        (b'G', 0.0009310209736100016),
        (b'J', 0.0008814561018445294),
        (b'H', 0.0008752446473266058),
        (b'O', 0.0008210528757671701),
        (b'W', 0.0008048270353938186),
        (b'j', 0.000617596049210692),
        (b'z', 0.0005762708620098124),
        (b'/', 0.000519607185080999),
        (b'<', 0.00044107665296153596),
        (b'>', 0.0004404428310719519),
        (b'K', 0.0003808001912620934),
        (b')', 0.0003314254660634964),
        (b'(', 0.0003307916441739124),
        (b'V', 0.0002556203680692448),
        (b'Y', 0.00025194420110965734),
        (b':', 0.00012036277683200988),
        (b'Q', 0.0001000170941763620),
    ]);
    let mut result = DecryptionScore {
        score: 0.,
        decrypted_string: String::from(""),
        key: 0,
    };
    (0u8..=255).for_each(|enc_byte| {
        let xor: Vec<u8> = encrypted_bytes.xor(&vec![enc_byte]);
        let score = xor
            .iter()
            .map(|byte| match weights.get(byte) {
                Some(&value) => value,
                None => 0.,
            })
            .sum::<f64>();
        let xor_f = xor.iter().map(|&c| c as char).collect::<String>();
        if score > 250. {
            println!("{enc_byte}: {score} {xor_f:?}");
        }
        if score > result.score {
            result.score = score;
            result.decrypted_string = xor.into_iter().map(|ch| ch as char).collect::<String>();
            result.key = enc_byte;
        }
    });
    result
}

pub fn find_best_normalized_keysize(test_bytes: &[u8]) -> usize {
    let mut best_key_size = 2;
    let mut best_distance = usize::MAX;
    for key_size in 2..=40 {
        let chunks: Vec<&[u8]> = test_bytes.chunks(key_size).take(4).collect();
        let mut hamming_distance: usize = 0;

        for i in 0..4 {
            for j in 0..4 {
                hamming_distance += chunks[i].hamming_distance(chunks[j]);
            }
        }

        hamming_distance /= key_size;

        if hamming_distance < best_distance {
            best_key_size = key_size;
            best_distance = hamming_distance;
        }
    }
    best_key_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use operations::{Transpose, Xor};
    use serialize::{Decode, Encode};

    use std::fs::read_to_string;

    #[test]
    fn ch1_hex_to_base64() {
        let initial = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(initial.bytes_from_hex().to_base64(), expected);
    }

    #[test]
    fn ch2_fixed_xor() {
        let initial = String::from("1c0111001f010100061a024b53535009181c").bytes_from_hex();
        let other = String::from("686974207468652062756c6c277320657965").bytes_from_hex();
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
                .bytes_from_hex();
        let decoding = find_best_decryption(&initial);
        assert_eq!(
            decoding.decrypted_string,
            String::from("Cooking MC's like a pound of bacon")
        );
    }

    #[test]
    fn ch4_single_character_xor() {
        let mut result = DecryptionScore {
            score: 0.,
            decrypted_string: String::from(""),
            key: 0,
        };
        read_to_string("data/s1c4")
            .unwrap()
            .lines()
            .for_each(|line| {
                let candidate = find_best_decryption(&String::from(line.trim()).bytes_from_hex());
                if candidate.score > result.score {
                    result = candidate
                }
            });

        assert_eq!(result.decrypted_string, "Now that the party is jumping\n");
    }

    #[test]
    fn ch5_repeating_key_xor() {
        let initial = String::from(
            "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal",
        )
        .into_bytes();
        let key = String::from("ICE").into_bytes();
        let result: Vec<u8> = initial.xor(&key);
        assert_eq!(
            result.to_hex(),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }

    #[test]
    fn ch6_breaking_repeating_key_xor() {
        let input = read_to_string("data/s1c6")
            .unwrap()
            .replace('\n', "")
            .bytes_from_base64();
        let key_size = find_best_normalized_keysize(&input);
        let transposed = input.transpose(key_size);
        let keys = transposed
            .iter()
            .map(|chunk| find_best_decryption(chunk).key)
            .collect::<Vec<u8>>();
        //let key_s = keys.iter().map(|&b| b as char).collect::<String>();
        //let decryption = input
        //    .xor(&keys)
        //    .iter()
        //    .map(|&b| b as char)
        //    .collect::<String>();
        assert_eq!(
            keys.iter().map(|&b| b as char).collect::<String>(),
            String::from("Terminator X: Bring the noise")
        );
    }
}
