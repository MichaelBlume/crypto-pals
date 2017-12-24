#[macro_use] extern crate lazy_static;

use std::io::{self, Read, Write};

const NUM_WINDOWS: usize = 128;

lazy_static! {
    static ref B64_TABLE: [u8; 64] = {
        let mut table = [0; 64];
        for i in 0..26 {
            table[i] = i as u8 + 'A' as u8;
        }
        for i in 0..26 {
            table[i + 26] = i as u8 + 'a' as u8
        }
        for i in 0..10 {
            table[i + 52] = i as u8 + '0' as u8;
        }
        table[62] = '+' as u8;
        table[63] = '/' as u8;
        table
    };
}

fn main() {
    let mut buffer = [0; 6 * NUM_WINDOWS];
    let mut out_buffer = [0; 4 * NUM_WINDOWS];

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(l) => {
                if l > 0 {
                    let actual_l = if (buffer[l - 1] as char).is_alphanumeric() {
                        l
                    } else {
                        l - 1
                    };
                    print_as_hex(actual_l, &buffer, &mut out_buffer)
                        .expect("should be able to print as hex");
                }
                if l == 6 * NUM_WINDOWS {
                    continue;
                }
            }
            Err(l) => panic!("=( {}", l),
        }
        break;
    }
}

fn hex_byte_to_nibble(hex_byte: u8) -> u8 {
    if hex_byte > 96 {
        // lower case letter
        hex_byte - 87
    } else if hex_byte > 64 {
        // upper case letter
        hex_byte - 55
    } else if hex_byte > 47 {
        // digit or :;<=>?@
        hex_byte - 48
    } else {
        panic!("this program is apparently (?) only meant to deal with \
                alphanumerics (and not, e.g., a space (' '), which underflows)");
    }
}

fn convert_to_hex<'a>(l: usize, in_buffer: &[u8], out_buffer: &'a mut [u8]) -> &'a [u8] {
    let triplet_count = (l + 5) / 6;
    for i in 0..triplet_count {
        let index = i * 6;
        let mut x: u32 = 0;
        for offset in 0..6 {
            x <<= 4;
            let next = index + offset;
            if next < l {
                let nibble = hex_byte_to_nibble(in_buffer[next]) as u32;
                x += nibble;
            }
        }
        for sextet in 0..4 {
            let shift = (3 - sextet) * 6;
            let shifted = x >> shift;
            let char_val = shifted & 63;
            let out_index = i * 4 + sextet ;

            if (l - index) / 2 >= sextet {
                out_buffer[out_index] = B64_TABLE[char_val as usize];
            } else {
                out_buffer[out_index] = '=' as u8;
            }

        }

    }
    &out_buffer[0..(triplet_count * 4)]
}

fn print_as_hex(l: usize, in_buffer: &[u8], out_buffer: &mut [u8]) -> Result<usize, io::Error> {
    io::stdout().write(convert_to_hex(l, in_buffer, out_buffer))
}

#[cfg(test)]
mod tests { // run with `cargo test`
    use super::{B64_TABLE, NUM_WINDOWS, convert_to_hex, print_as_hex};

    #[test]
    fn test_main() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let mut out_buffer = [0; 4 * NUM_WINDOWS];
        let result = convert_to_hex(s.len(), s.as_bytes(), &mut out_buffer);
        assert_eq!(b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_vec(), // 'b'-prefixed byte literals are a thing
                   result);
    }

}
