use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(number: &str, option: &str) -> String {
    let x: String;
    if String::from(option) == "arabic" {
        x = arabic_convert(number).unwrap().to_string();
    } else if String::from(option) == "roman" {
        x = roman_convert(number.parse().unwrap()).unwrap().to_string();
    } else {
        x = "ERROR".to_string();
    }
    return x.to_string();
}

pub const CONVERTIBLE: [(&str, u32); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

pub fn arabic_convert(number_to_convert: &str) -> Result<u32, String> {
    let mut arabic_result = 0;

    let normalized_array = get_normalized_array(&number_to_convert);

    let mut curr_idx = 0;
    let max_idx = normalized_array.len() - 1;

    while curr_idx <= max_idx {
        if curr_idx + 1 <= max_idx {
            let key_candidate = normalized_array.get(curr_idx).unwrap().to_owned()
                + normalized_array.get(curr_idx + 1).unwrap();
            let symbol = get_symbol(&key_candidate);

            if symbol.is_some() {
                arabic_result = arabic_result + symbol.unwrap().1;
                curr_idx = curr_idx + 2;

                continue;
            }
        }

        let key_candidate = normalized_array.get(curr_idx).unwrap();
        let symbol = get_symbol(&key_candidate);

        if symbol.is_some() {
            arabic_result = arabic_result + symbol.unwrap().1;
            curr_idx = curr_idx + 1;

            continue;
        }

        return Err(format!("Symbol '{}' not found", key_candidate.to_string()));
    }

    Ok(arabic_result)
}

fn get_normalized_array(number_to_convert: &str) -> Vec<String> {
    let normalized_string = number_to_convert.to_ascii_uppercase();
    let normalized_string = normalized_string.trim();

    if normalized_string == "" {
        return Vec::new();
    }

    normalized_string
        .split("")
        .filter(|s| s.to_string() != "")
        .map(|s| s.to_string())
        .collect()
}

fn get_symbol(symbol: &str) -> Option<(String, u32)> {
    for s in CONVERTIBLE.iter() {
        if symbol == s.0 {
            return Some((s.0.to_string(), s.1));
        }
    }

    None
}

pub fn roman_convert(number_to_convert: u32) -> Result<String, String> {
    if number_to_convert > 3999 {
        return Err(format!(
            "Invalid conversion of '{}' above maximum limit of 3999",
            number_to_convert
        ));
    }

    let mut roman_number = String::new();
    let mut to_convert = number_to_convert;

    while to_convert > 0 {
        'inner: for s in CONVERTIBLE.iter() {
            if s.1 > to_convert {
                continue 'inner;
            }

            roman_number = roman_number + s.0;
            to_convert = to_convert - s.1;

            break 'inner;
        }
    }

    Ok(roman_number)
}
