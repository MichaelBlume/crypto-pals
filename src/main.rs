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
}
