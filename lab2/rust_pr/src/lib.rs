pub mod internals;
pub mod criterions;

pub const UKR_ALPHABET_WITH_G: [char; 33] = [
    'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];
pub const UKR_ALPHABET: [char; 32] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];

pub const THRESHOLD: u64 = 10;
pub const L1: usize = 10;
pub const L2: usize = 100;
pub const L3: usize = 1000;
/// N1 is only for L1, L2, L3 values
pub const N1:usize = 10_000;
/// N2 is only for L3 value
pub const L4: usize = 10_000;
pub const N2:usize = 1000;