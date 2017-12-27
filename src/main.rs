use std::io::{self, Read, Write};
use std::char;

const NUM_WINDOWS: usize = 128;

fn b64_alphabet() -> &'static [u8] {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes()
}

fn main() {
    let mut buffer = [0; 6 * NUM_WINDOWS];
    let mut out_buffer = [0; 4 * NUM_WINDOWS];
    let b64_table = b64_alphabet();

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(l) => {
                if l > 0 {
                    let actual_l = if (buffer[l - 1] as char).is_alphanumeric() {
                        l
                    } else {
                        l - 1
                    };
                    let out_len = convert_to_hex(actual_l, &buffer, &mut out_buffer, &b64_table);
                    io::stdout().write(&out_buffer[0..out_len]).expect("write to stdout");
                }
                if l != 6 * NUM_WINDOWS {
                    break;
                }
            }
            Err(l) => panic!("=( {}", l),
        }
    }
}

fn convert_to_hex(l: usize, in_buffer: &[u8], out_buffer: &mut [u8], b64_table: &[u8]) -> usize {
    let triplet_count = (l + 5) / 6;
    for i in 0..triplet_count {
        let index = i * 6;
        let mut x: u32 = 0;
        for offset in 0..6 {
            x = x << 4;
            let next = index + offset;
            if next < l {
                x += (in_buffer[next] as char).to_digit(16).expect("Invalid hex char!");
            }
        }
        for sextet in 0..4 {
            let shift = (3 - sextet) * 6;
            let shifted = x >> shift;
            let char_val = shifted & 63;
            let out_index = i * 4 + sextet;

            if (l - index) / 2 >= sextet {
                out_buffer[out_index] = b64_table[char_val as usize];
            } else {
                out_buffer[out_index] = b'=';
            }
        }
    }
    triplet_count * 4
}

fn xor_buffers(s1: &[u8], s2: &[u8], out_buffer: &mut [u8]) {
    for i in 0..s1.len() {
        let nibble1 = (s1[i] as char).to_digit(16).expect("hex");
        let nibble2 = (s2[i] as char).to_digit(16).expect("hex2");
        let nibble_result = nibble1 ^ nibble2;
        out_buffer[i] = char::from_digit(nibble_result, 16).expect("should be hex") as u8;
    }
}

fn hex_byte_to_nibble(b: u8) -> u8 {
    (b as char).to_digit(16).expect("hex") as u8
}

fn hex_to_bytestring(s: &[u8]) -> Vec<u8> {
    let ret_len = s.len() / 2;
    let mut ret = Vec::with_capacity(ret_len);
    for i in 0..ret_len {
        ret.push(hex_byte_to_nibble(s[i * 2]) * 16 + hex_byte_to_nibble(s[i * 2 + 1]));
    }
    ret
}

fn create_scoring_table() -> [u8; 256] {
    let mut ret = [0; 256];

    ret[b' ' as usize] = 167;

    let u = |ret: &mut [u8], c: u8, score: u8| {
        ret[c as usize] = score;
        ret[(c + b'A' - b'a') as usize] = score;
    };

    u(&mut ret, b'e', 127);
    u(&mut ret, b't', 91);
    u(&mut ret, b'a', 82);
    u(&mut ret, b'o', 76);
    u(&mut ret, b'i', 70);
    u(&mut ret, b'n', 67);
    u(&mut ret, b's', 63);
    u(&mut ret, b'h', 61);
    u(&mut ret, b'r', 60);
    u(&mut ret, b'd', 43);
    u(&mut ret, b'l', 40);
    u(&mut ret, b'c', 28);
    u(&mut ret, b'u', 28);

    ret
}

fn score_string_with_key(cipher_string: &Vec<u8>, key: u8, score_table: &[u8]) -> i32 {
    let mut new_score = 0;
    for i in 0..cipher_string.len() {
        let char_ind = (cipher_string[i] ^ key) as usize;
        let char_score = score_table[char_ind] as i32;
        new_score += char_score;
    }
    new_score
}

fn decode_string_with_key(cipher_string: &mut Vec<u8>, key: u8) {
    for i in 0..cipher_string.len() {
        cipher_string[i] ^= key
    }

}

fn decode_hex_cipher(s: &[u8]) -> Vec<u8> {
    let mut cipher_string = hex_to_bytestring(s);
    let mut best_score = 0;
    let mut best_key = 0;
    let score_table = create_scoring_table();
    let mut key = 0;
    loop {
        let new_score = score_string_with_key(&cipher_string, key, &score_table);
        if new_score > best_score {
            best_key = key;
            best_score = new_score;
        }
        if key == 255 {
            break;
        }
        key += 1;
    }
    decode_string_with_key(&mut cipher_string, best_key);
    cipher_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_b64() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let mut out_buffer = [0; 4 * NUM_WINDOWS];
        let out_size = convert_to_hex(s.len(), s.as_bytes(), &mut out_buffer, b64_alphabet());
        assert_eq!(str::from_utf8(&out_buffer[0..out_size]).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }

    #[test]
    fn test_xor() {
        let s1 = "1c0111001f010100061a024b53535009181c";
        let s2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let mut out_buffer = [0; 50];
        xor_buffers(s1.as_bytes(), s2.as_bytes(), &mut out_buffer);
        assert_eq!(str::from_utf8(&out_buffer[0..s1.len()]).unwrap(), expected);
    }

    #[test]
    fn test_decode_single_byte() {
        let result = decode_hex_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes());
        assert_eq!("Cooking MC's like a pound of bacon", str::from_utf8(&result).expect("utf8"));
    }
}
