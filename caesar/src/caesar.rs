use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(Debug, PartialEq)]
pub struct KeyError;

const KEY_ERROR_MSG: &'static str = "the key parameter must be a positive number between 0 - 999999.";

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", KEY_ERROR_MSG)
    }
}

impl Error for KeyError {
    fn description(&self) -> &str {
        KEY_ERROR_MSG
    }
}

pub fn caesar(input: &str, key: i32, dir: Mode) -> Result<String, KeyError> {
    if key.is_negative() {
        return Err(KeyError);
    }
    if key > 999_999 {
        return Err(KeyError);
    }

    let alphabet_pos = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let mut alphabet_dic = HashMap::new();
    for (index, letter) in alphabet_pos.iter().enumerate() {
        alphabet_dic.insert(*letter, index);
    }

    let mut result = String::new();

    for ic in input.chars() {
        let ic_lower: Vec<_> = ic.to_lowercase().collect();
        match alphabet_dic.get(&ic_lower.get(0).unwrap()) {
            Some(index) => {
                let calc_index: usize;
                match dir {
                    Mode::Encrypt => calc_index = calc_index_forward(index, key),
                    Mode::Decrypt => calc_index = calc_index_backward(index, key)
                }
                let matched_char = alphabet_pos.get(calc_index).unwrap().to_string();
                if ic.is_uppercase() {
                    result.push_str(matched_char.to_uppercase().as_str());
                    continue;
                }
                result.push_str(matched_char.as_str());
            }
            None => result.push_str(ic.to_string().as_str())
        }
    }
    Ok(result)
}

fn calc_index_forward(letter_index: &usize, key: i32) -> usize {
    let li = *letter_index as i32;
    let result = (li + key) % 26;
    return result as usize;
}

fn calc_index_backward(letter_index: &usize, key: i32) -> usize {
    let li = *letter_index as i32;
    let mut result = (li - key) % 26;
    if result.is_negative() {
        result = 26 + result
    }
    return result as usize;
}


#[cfg(test)]
mod tests {
    use crate::caesar::{caesar, KeyError, Mode};

    #[test]
    fn it_encrypts_basic_string() {
        let result = caesar("ABC", 1, Mode::Encrypt).unwrap();
        assert_eq!("BCD", result);
    }

    #[test]
    fn it_decrypts_basic_string() {
        let result = caesar("BCD", 1, Mode::Decrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_ignores_but_keeps_non_alphabet_characters() {
        let result = caesar("(ABC)D", 1, Mode::Encrypt).unwrap();
        assert_eq!("(BCD)E", result);
    }

    #[test]
    fn it_respects_spaces() {
        let result = caesar("A B C", 1, Mode::Encrypt).unwrap();
        assert_eq!("B C D", result);
    }

    #[test]
    fn it_respects_multiline() {
        let result = caesar("A \n B \n C", 1, Mode::Encrypt).unwrap();
        assert_eq!("B \n C \n D", result);
    }

    #[test]
    fn it_respects_capitalization() {
        let result = caesar("ABC", 1, Mode::Encrypt).unwrap();
        assert_eq!("BCD", result);

        let result = caesar("abc", 1, Mode::Encrypt).unwrap();
        assert_eq!("bcd", result);
    }

    #[test]
    fn it_ignores_but_keeps_utf8_chars() {
        let result = caesar("ЗaЗ", 1, Mode::Encrypt).unwrap();
        assert_eq!("ЗbЗ", result)
    }

    #[test]
    fn it_handles_last_alpha_pos_encrypt() {
        let result = caesar("ABC", 26, Mode::Encrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_handles_last_alpha_pos_decrypt() {
        let result = caesar("ABC", 26, Mode::Decrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_upper_overflow() {
        let result = caesar("XY", 3, Mode::Encrypt).unwrap();
        assert_eq!("AB", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_lower_overflow() {
        let result = caesar("BC", 3, Mode::Decrypt).unwrap();
        assert_eq!("YZ", result);
    }

    #[test]
    // Two times the alphabet + 2 (forward).
    fn it_handles_upper_bound_overflow() {
        let result = caesar("ABC", 54, Mode::Encrypt).unwrap();
        assert_eq!("CDE", result);
    }

    #[test]
    // Two times the alphabet + 3 (backward).
    fn it_handles_lower_bound_overflow() {
        let result = caesar("ABC", 55, Mode::Decrypt).unwrap();
        assert_eq!("XYZ", result);
    }

    #[test]
    fn it_returns_same_on_no_key() {
        let result = caesar("ABC", 0, Mode::Encrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_returns_error_on_negative_key() {
        let result = caesar("ABC", -1, Mode::Encrypt).unwrap_err();
        assert_eq!(KeyError, result);
    }

    #[test]
    fn errors_key_has_display() {
        let error = KeyError {};
        assert_eq!("the key parameter must be a positive number between 0 - 999999.", format!("{}", error));
    }

    #[test]
    fn it_returns_error_on_max_key_size() {
        let result = caesar("ABC", 1_000_000, Mode::Encrypt).unwrap_err();
        assert_eq!(KeyError, result);
    }

    #[test]
    fn it_deals_with_max_key_size() {
        let result = caesar("ABC", 999_999, Mode::Encrypt).unwrap();
        assert_eq!("NOP", result);
    }
}
