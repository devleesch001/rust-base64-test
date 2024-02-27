use base64::prelude::*;

fn main() {
    for i in 0..=u8::MAX {
        let bytes = [0x00, 0x00, 0x00, i, 0x00];
        let (encoded, decoded) = encode_decode(&bytes);

        println!("{} : {:?} | {:08b} -> {} -> {:08b} | {:?} : {}", i, bytes, bytes[3], encoded, &decoded[3], &decoded, bytes == decoded[..]);
    }
}

fn encode_decode(input: &[u8]) -> (String, Vec<u8>) {
    let encoded = BASE64_STANDARD.encode(input);
    let decoded = BASE64_STANDARD.decode(encoded.as_bytes()).unwrap();

    return (encoded, decoded);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodes_decodes() {
        for i in 0..=u8::MAX {
            let bytes = [0x00, 0x00, 0x00, i, 0x00];
            let (encoded, decoded) = encode_decode(&bytes);

            assert_eq!(decoded, bytes, "Failed for base64 {}, bytes value: {:?}", encoded, &decoded[..]);
        }
    }
}

