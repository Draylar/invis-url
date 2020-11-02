use serde::{Deserialize, Serialize};

const ZERO: char = '​';
const ONE: char = '‍';
const SPACE: char = '‌';

#[derive(Debug,Serialize, Deserialize)]
pub struct ConversionData {
    pub(crate) url: String
}

pub(crate) fn url_to_invisible(url: String) -> String {
    if url.is_empty() {
        return url
    }

    let binary = string_to_binary(url);
    let replacements = binary_to_replacements(binary);
    format!("https://invis-url.draylar.dev/{}", replacements)
}

pub(crate) fn invisible_to_url(invisible: String) -> String {
    let binary = replacements_to_binary(invisible);
    println!("Coming back: {}", binary);
    let without_domain = binary.replace("https://invis-url.draylar.dev/", "");
    println!("Without domain: {}", without_domain);
    binary_to_string(without_domain)
}

/// Converts a binary string to invisible spaces.
fn binary_to_replacements(binary: String) -> String {
    let mut invis = binary.clone();
    invis = invis.replace("0", &*ZERO.to_string());
    invis = invis.replace("1", &*ONE.to_string());
    invis = invis.replace(" ", &*SPACE.to_string());
    invis
}

fn replacements_to_binary(invis: String) -> String {
    let mut binary = invis.clone();
    binary = binary.replace(&*ONE.to_string(), "1");
    binary = binary.replace(&*ZERO.to_string(), "0");
    binary = binary.replace(&*SPACE.to_string(), " ");
    binary
}

/// Converts a `String` into binary.
///
/// `to_binary("Hello")` -> `01001000 01100101 01101100 01101100 01101111`
fn string_to_binary(string: String) -> String {
    let mut binary = "".to_string();

    for c in string.clone().as_bytes() {
        binary += &format!("0{:b} ", c);
    }

    // Attempt to trim space from final index
    if binary.len() >= 1 {
        binary.truncate(binary.len() - 1);
    }

    binary
}

/// Converts a String with binary contents to a String with ASCII characters.
///
/// `from_binary("01001000 01100101 01101100 01101100 01101111")` -> `Hello`
fn binary_to_string(string: String) -> String {
    if string.is_empty() {
        return string;
    }

    let split = string.split(" ");
    let mut to = "".to_string();

    for c in split {
        to += bytes_to_string(binary_to_bytes(c.to_string()) as u8).as_str();
    }

    to
}

fn binary_to_bytes(binary: String) -> usize {
    let mut bytes: usize = 0;

    for i in 0..binary.len() {
        let at = char::to_digit(binary.chars().nth(binary.len() - 1 - i).unwrap(), 10).unwrap() as usize;
        let cur = (2_u32.pow(i as u32) * at as u32) as usize;
        bytes += cur;
    }

    bytes
}

fn bytes_to_string(bytes: u8) -> String {
    (bytes as char).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_binary() {
        assert_eq!(string_to_binary("Hello".to_string()), "01001000 01100101 01101100 01101100 01101111");
        assert_eq!(string_to_binary("H".to_string()), "01001000");
        assert_eq!(string_to_binary(" ".to_string()), "0100000");
        assert_eq!(string_to_binary("".to_string()), "");
    }

    #[test]
    fn test_binary_to_string() {
        assert_eq!(binary_to_string("1100111 1101111 1101111 1100111 1101100 1100101 101110 1100011 1101111 1101101".to_string()), "google.com");
        assert_eq!(binary_to_string("01001000 01100101 01101100 01101100 01101111".to_string()), "Hello");
        assert_eq!(binary_to_string("01001000".to_string()), "H");
        assert_eq!(binary_to_string("0100000".to_string()), " ");
        assert_eq!(binary_to_string("".to_string()), "");
    }


    #[test]
    fn test_binary_to_bytes() {
        assert_eq!(binary_to_bytes("01000100".to_string()), 68); // 68 -> D
        assert_eq!(binary_to_bytes("01100001".to_string()), 97); // 97 -> a
        assert_eq!(binary_to_bytes("01100111".to_string()), 103); // 103 -> g
    }

    #[test]
    fn test_bytes_to_string() {
        assert_eq!(bytes_to_string(68), "D");
        assert_eq!(bytes_to_string(97), "a");
        assert_eq!(bytes_to_string(103), "g");
    }

    #[test]
    fn test_conversion() {
        assert_eq!(invisible_to_url(url_to_invisible("google.com".to_string())), "google.com");
        assert_eq!(invisible_to_url(url_to_invisible("https://draylar.dev".to_string())), "https://draylar.dev");
    }
}