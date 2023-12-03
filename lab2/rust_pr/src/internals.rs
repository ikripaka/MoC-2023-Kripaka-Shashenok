use chrono::Local;
use dotenv::dotenv;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::{UKR_ALPHABET, UKR_ALPHABET_REVERSE_MAP};
use regex::Regex;

// =================== TEXT DISTORTION BEGINNING ===================
#[inline]
fn generate_random_l_gram(l: usize, alphabet_len: usize, alphabet: &[char]) -> String {
    (0..l)
        .map(|x| alphabet[thread_rng().gen_range(0..alphabet_len)])
        .collect()
}

#[inline]
fn slice_u8_to_string(slice: &[u8], alphabet: &[char]) -> String {
    slice.iter().map(|x| alphabet[*x as usize]).collect()
}

#[inline]
fn gen_random_vec(len: usize, gen_range: u8) -> Vec<u8> {
    (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u8>>()
}

#[inline]
fn gen_random_in_vec_ref(vec: &mut Vec<u8>, len: usize, gen_range: u8) {
    *vec = (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u8>>()
}

#[inline]
fn slice_into_string(slice: &[u8], alphabet: &[char]) -> String {
    slice.iter().map(|x| alphabet[*x as usize]).collect()
}

/// Vigenère cipher
/// (a) DISTORSION OF THE TEXT
pub fn vigenere_cipher_distortion(
    r: usize,
    l_grams: &[String],
    alphabet: &[char],
) -> (Vec<String>, Vec<u8>) {
    let distore_n_grams =
        |vec: &mut Vec<String>, slice: &[String], key: &[u8], r: usize, alphabet_len: u8| {
            for l_gram in slice {
                vec.push(slice_into_string(
                    &l_gram
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            (*UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap() + key[i % r]) % alphabet_len
                        })
                        .collect::<Vec<u8>>(),
                    alphabet,
                ))
            }
        };

    let alphabet_len = alphabet.len() as u8;
    let mut key = gen_random_vec(r, alphabet_len);
    let (mut left, mut right) = (Vec::new(), Vec::new());
    rayon::join(
        || {
            distore_n_grams(
                &mut left,
                &l_grams[0..(l_grams.len() >> 1)],
                &key,
                r,
                alphabet_len as u8,
            )
        },
        || {
            distore_n_grams(
                &mut right,
                &l_grams[(l_grams.len() >> 1)..],
                &key,
                r,
                alphabet_len as u8,
            )
        },
    );
    left.extend_from_slice(&right);
    (left, key)
}

/// affine substitution
/// (б) DISTORSION OF THE TEXT
pub fn generate_affine_distortion(
    l: usize,
    l_grams: &[String],
    alphabet: &[char],
) -> (Vec<String>, (Vec<u8>, Vec<u8>)) {
    let distore_n_grams =
        |vec: &mut Vec<String>, slice: &[String], a: &[u8], b: &[u8], alphabet_len: u16| {
            for l_gram in slice {
                vec.push(slice_into_string(
                    &l_gram
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            ((a[i] as u16 * *UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap() as u16
                                + b[i] as u16)
                                % alphabet_len) as u8
                        })
                        .collect::<Vec<u8>>(),
                    alphabet,
                ))
            }
        };

    let alphabet_len = alphabet.len() as u8;
    let (mut a, mut b) = (Vec::new(), Vec::new());
    rayon::join(
        || gen_random_in_vec_ref(&mut a, l, alphabet_len),
        || gen_random_in_vec_ref(&mut b, l, alphabet_len),
    );
    let (mut left, mut right) = (Vec::new(), Vec::new());
    rayon::join(
        || {
            distore_n_grams(
                &mut left,
                &l_grams[0..(l_grams.len() >> 1)],
                &a,
                &b,
                alphabet_len as u16,
            )
        },
        || {
            distore_n_grams(
                &mut right,
                &l_grams[(l_grams.len() >> 1)..],
                &a,
                &b,
                alphabet_len as u16,
            )
        },
    );
    left.extend_from_slice(&right);
    (left, (a, b))
}

/// `generate_random_n_l_gram` -- generates `n` random `l`-grams
/// (в) DISTORSION OF THE TEXT
pub fn generate_random_n_l_grams(l: usize, n: usize, alphabet: &[char]) -> Vec<String> {
    let mut res = Vec::new();
    let alphabet_len = alphabet.len();
    for _ in 0..n {
        res.push(generate_random_l_gram(l, alphabet_len, alphabet));
    }
    res
}

/// `recurrent_generation_n_l_grams` -- generates `n` `l`-grams via recurrent formula
/// (г) DISTORSION OF THE TEXT
pub fn recurrent_generation_n_l_grams(l: usize, n: usize, alphabet: &[char]) -> Vec<String> {
    let alphabet_len = alphabet.len() as u8;
    let mut res = Vec::new();

    let (mut s_1, mut s_2) = (Vec::new(), Vec::new());
    rayon::join(
        || gen_random_in_vec_ref(&mut s_1, l, alphabet_len),
        || gen_random_in_vec_ref(&mut s_2, l, alphabet_len),
    );
    res.push(slice_u8_to_string(&s_1, alphabet));
    res.push(slice_u8_to_string(&s_2, alphabet));
    for _ in 2..n {
        let mut s_0 = Vec::new();
        let mut str = String::new();
        for i in 0..s_1.len() {
            let new_index = (s_1[i] + s_2[i]) % alphabet_len;
            s_0.push(new_index);
            str.push(alphabet[new_index as usize]);
        }
        res.push(str);
        s_2 = s_1.clone();
        s_1 = s_0;
    }
    res
}

// =================== TEXT DISTORTION ENDING ===================

///
pub fn make_frequency_table_from_file(filepath: &str, chunks: usize) -> HashMap<String, u64> {
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);

    let sub_strings = make_n_gram_on_content(chunks, content);

    let mut map = HashMap::new();

    for x in sub_strings {
        match map.get(&x) {
            None => {
                map.insert(x.clone(), 1);
            }
            Some(val) => {
                map.insert(x.clone(), *val + 1);
            }
        }
    }
    map
}

pub fn make_frequency_table(content: &str, chunks: usize) -> HashMap<String, u64> {
    let sub_strings = make_n_gram_on_content_from_str(chunks, content);

    let mut map = HashMap::new();

    for x in sub_strings {
        match map.get(&x) {
            None => {
                map.insert(x.clone(), 1);
            }
            Some(val) => {
                map.insert(x.clone(), *val + 1);
            }
        }
    }
    map
}

/// `make_n_gram_on_content` -- constructs from given content given length symbols chunks
pub fn make_n_gram_on_content(chunks: usize, content: String) -> Vec<String> {
    //trim string to specific size
    let mut content = content;
    if content.chars().count() % chunks != 0 {
        let str_len = content.chars().count();
        for _ in 0..(str_len % chunks) {
            content.pop().unwrap();
        }
    }

    let mut chars = content.chars();
    (0..)
        .map(|_| chars.by_ref().take(chunks).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>()
}

/// `make_n_gram_on_content` -- constructs from given content given length symbols chunks
pub fn make_n_gram_on_content_from_str(chunks: usize, content: &str) -> Vec<String> {
    //trim string to specific size
    let mut content = content.to_string();
    if content.chars().count() % chunks != 0 {
        let str_len = content.chars().count();
        for _ in 0..(str_len % chunks) {
            content.pop().unwrap();
        }
    }

    let mut chars = content.chars();
    (0..)
        .map(|_| chars.by_ref().take(chunks).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>()
}

pub fn is_n_gram_prohibited_with_ngrams(
    n_gram: &str,
    frequency_table: &HashMap<String, u64>,
    threshold: u64,
) -> bool {
    match frequency_table.get(n_gram) {
        None => true,
        Some(val) => {
            if *val < threshold {
                return true;
            }
            false
        }
    }
}

pub fn is_n_gram_prohibited_with_custom_l_grams(
    n_gram: &str,
    frequency_table: &HashMap<String, u64>,
    threshold: u64,
    chunks: usize,
) -> bool {
    for i in 0..chunks {
        let mut counter = n_gram.chars().into_iter().count() - i;
        let mut char_iter = n_gram.chars().into_iter();
        let _skip = (0..i)
            .into_iter()
            .map(|_| char_iter.next().unwrap())
            .collect::<String>();
        while counter > chunks {
            match frequency_table.get(
                &(0..chunks)
                    .into_iter()
                    .map(|x| match char_iter.next() {
                        None => {
                            println!("hi bug!");
                            'a'
                        }
                        Some(c) => c,
                    })
                    .collect::<String>(),
            ) {
                None => return true,
                Some(val) => {
                    if *val < threshold {
                        return true;
                    }
                    // false
                    // then looking for another chunk_grams
                }
            }
            counter -= chunks;
        }
    }
    false
}

pub fn make_n_gram_on_file_content(filepath: &str, chunks: usize) -> Vec<String> {
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    make_n_gram_on_content(chunks, content)
}

/// `make_n_gram_on_alphabet` -- constructs from alphabet n-grams
/// it's like all possible combinations of our alphabet
/// n = 1 -- 'аа', 'аб', 'ав', 'аг', ...
pub fn make_n_gram_on_alphabet(n: usize, alphabet: &[char]) -> Vec<String> {
    let mut vec = alphabet
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    for _ in 1..n {
        let mut tmp = Vec::new();
        for x in alphabet {
            let mut bunches_of_n_gram = vec.clone();
            bunches_of_n_gram = bunches_of_n_gram
                .iter()
                .map(|y| (y.clone()) + x.to_string().as_str())
                .collect();
            tmp.extend_from_slice(&bunches_of_n_gram);
        }
        vec = tmp
    }
    vec
}

pub fn make_probability_table(freq_table: &HashMap<String, u64>) -> HashMap<String, f64> {
    let sum = freq_table.iter().fold(0, |acc, (_, val)| acc + val);
    freq_table
        .iter()
        .map(|(key, val)| (key.clone(), (*val as f64 / (sum as f64))))
        .collect()
}

pub fn calculate_entropy(prob_table: &HashMap<String, f64>) -> f64 {
    prob_table.iter().fold(0., |acc, (key, val)| {
        println!("acc: {acc}, val: {val}");
        if *val == 0. {
            acc
        } else {
            acc - *val * (*val).log2()
        }
    })
}

pub fn calculate_probs(h0: u64, h1: u64, total_l_grams_amount: usize) -> (f64, f64) {
    println!("total_l_grams: {total_l_grams_amount}");
    let all_cases = total_l_grams_amount as f64;
    let (p_h_0, p_h_1) = (h0 as f64 / all_cases, h1 as f64 / all_cases);
    let p_h_0_1 = p_h_0 * p_h_1;
    println!("p_h_0_1: {p_h_0_1}, p_h_0: {p_h_0}, p_h_1:{p_h_1}");
    (p_h_0_1 / p_h_0, p_h_0_1 / p_h_1)
}

/// **format_file** -- formats file and deletes all redundant symbol
/// except:
///     * ukrainian alphabet
///     * "ґ" replaced into -> "г"
pub fn format_file(in_filepath: &str, out_filepath: &str) {
    let re = Regex::new(r"[^абвгґдеєжзиіїйклмнопрстуфхцчшщьюя]+").unwrap();

    let to_filter = File::open(in_filepath).unwrap();
    let mut reader = BufReader::new(to_filter);

    let to_write = File::create(out_filepath).unwrap();
    let mut writer = BufWriter::new(to_write);

    let mut buf = String::new();
    while reader.read_line(&mut buf).unwrap() != 0 {
        reader.read_line(&mut buf).unwrap();
        buf = buf.to_lowercase();
        buf = buf.replace("ґ", "г");
        buf = re.replace_all(&buf, "").to_string();

        writer.write(buf.as_bytes()).unwrap();
        buf.clear();
    }
    writer.flush().unwrap();
}

#[test]
fn generate_random_n_gram_test() {
    let rand_gen_n_gram = generate_random_n_l_grams(100, 10, &UKR_ALPHABET);

    println!("random n_gram: {:?}", rand_gen_n_gram)
}

#[test]
fn recurrent_gen_test() {
    let recur = recurrent_generation_n_l_grams(10, 10, &UKR_ALPHABET);

    println!("recurrent n_gram: {:?}", recur)
}

#[test]
fn affine_distortion_gen_test() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let chunks = 2;
    let n_grams = make_n_gram_on_file_content(&filepath, chunks);
    let affine_distortion = generate_affine_distortion(chunks, &n_grams[0..10], &UKR_ALPHABET);
    println!(
        "original: {:?} \n\t\t affine distorted n_gram: {:?} \n\t\t{:?}",
        &n_grams[0..10],
        affine_distortion,
        &n_grams[0..10]
            .iter()
            .map(|c| c
                .chars()
                .map(|c| *UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>()
    )
}

#[test]
fn vigenere_distortion_test() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let chunks = 2;
    let n_grams = make_n_gram_on_file_content(&filepath, chunks);
    let vigenere_cipher_distortion = vigenere_cipher_distortion(1, &n_grams[0..10], &UKR_ALPHABET);
    println!(
        "original: {:?} \n\t\t vigenere cipher distortion n_gram: {:?} \n\t\t{:?}",
        &n_grams[0..10],
        vigenere_cipher_distortion,
        &n_grams[0..10]
            .iter()
            .map(|c| c
                .chars()
                .map(|c| *UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>()
    )
}

#[test]
fn custom_forbidden_grams_test() {
    let not_forbidden = "типопродалисьсподіваючисьзатенагородипанипольськіневипускалисвоїхдочокздо";
    let forbidden = "бтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрп";

    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let (chunks, threshold) = (2, 10);
    let freq_table = make_frequency_table_from_file(&filepath, chunks);
    let time_prev = Local::now();
    println!(
        "is_forbidden forbidden: {:?} \n\t\t is_forbidden real: {:?}",
        // is_n_gram_forbidden_with_custom_l_grams(forbidden, &freq_table,threshold, chunks),
        "",
        is_n_gram_prohibited_with_custom_l_grams(not_forbidden, &freq_table, threshold, chunks)
    );
    let time_after = Local::now();
    println!("{}", (time_after - time_prev).num_milliseconds())
}
