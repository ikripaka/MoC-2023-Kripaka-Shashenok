use lazy_static::lazy_static;
use std::collections::HashMap;

pub mod criterions;
pub mod internals;

pub const UKR_ALPHABET: [char; 32] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
    'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];

lazy_static! {
    pub static ref UKR_ALPHABET_REVERSE_MAP: HashMap<char, u8> = {
        let mut m = HashMap::new();
        for i in 0..UKR_ALPHABET.len() {
            m.insert(UKR_ALPHABET[i], i as u8);
        }
        m
    };
}

pub const THRESHOLD: u64 = 10;
pub const L_BIGRAM: usize = 2;
pub const L_THREE_GRAM: usize = 3;
pub const R1: usize = 1;
pub const R2: usize = 5;
pub const R3: usize = 10;
pub const L1: usize = 10;
pub const L2: usize = 100;
pub const L3: usize = 1000;
/// N1 is only for L1, L2, L3 values
pub const N1: usize = 10_000;
/// N2 is only for L3 value
pub const L4: usize = 10_000;
pub const N2: usize = 1000;
