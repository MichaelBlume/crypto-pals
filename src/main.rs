use std::io::{self, Read, Write};

const NUM_WINDOWS: usize = 128;

fn main() {
    let mut buffer = [0; 6 * NUM_WINDOWS];
    let mut out_buffer = [0; 4 * NUM_WINDOWS];
    let mut b64_table = [0; 64];
    assemble_b64_table(&mut b64_table);

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(l) => {
                if l > 0 {
                    let actual_l = if (buffer[l - 1] as char).is_alphanumeric() {
                        l
                    } else {
                        l - 1
                    };
                    print_as_hex(actual_l, &buffer, &mut out_buffer, &b64_table);
                }
                if l != 6 * NUM_WINDOWS {
                    break;
                }
            }
            Err(l) => panic!("=( {}", l),
        }
    }
}

fn test_main() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let mut out_buffer = [0; 4 * NUM_WINDOWS];
    let mut b64_table = [0; 64];
    assemble_b64_table(&mut b64_table);
    print_as_hex(s.len(), s.as_bytes(), &mut out_buffer, &b64_table);
}


fn hex_byte_to_nibble(hex_byte: u8) -> u8 {
    if hex_byte > 96 {
        // lower case letter
        return hex_byte - 87;
    } else if hex_byte > 64 {
        // upper case letter
        return hex_byte - 55;
    } else {
        // digit
        return hex_byte - 48;
    }
}

fn assemble_b64_table(table: &mut [u8]) {
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
}

fn print_as_hex(l: usize, in_buffer: &[u8], out_buffer: &mut [u8], b64_table: &[u8]) {
    let triplet_count = (l + 5) / 6;
    for i in 0..triplet_count {
        let index = i * 6;
        let mut x: u32 = 0;
        for offset in 0..6 {
            x = x << 4;
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
                out_buffer[out_index] = b64_table[char_val as usize];
            } else {
                out_buffer[out_index] = '=' as u8;
            }

        }

    }
    io::stdout().write(&out_buffer[0..(triplet_count * 4)]);
}

