use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::ops::Range;
use std::str::Chars;

use chrono::Local;
use dotenv::dotenv;
use rand::{thread_rng, Rng};
use regex::Regex;

use crate::{
    L1, L2, L3, L4, L_BIGRAM, UKR_ALPHABET, UKR_ALPHABET_REVERSE_MAP, UKR_BIGRAM_REVERSE_MAP,
};

// =================== TEXT DISTORTION BEGINNING ===================
#[inline]
pub fn gen_random_l_gram_char_alphabet(
    l: usize,
    alphabet_len: usize,
    alphabet: &[char],
) -> String {
    (0..l)
        .map(|x| alphabet[thread_rng().gen_range(0..alphabet_len)])
        .collect()
}

#[inline]
pub fn gen_random_l_gram_string_alphabet(
    l: usize,
    alphabet_len: usize,
    alphabet: &[String],
) -> String {
    (0..l)
        .map(|x| alphabet[thread_rng().gen_range(0..alphabet_len)].clone())
        .collect()
}

#[inline]
fn slice_u8_to_string(slice: &[u8], alphabet: &[char]) -> String {
    slice.iter().map(|x| alphabet[*x as usize]).collect()
}

#[inline]
fn slice_u16_to_string(slice: &[u16], alphabet: &[String]) -> String {
    slice.iter().map(|x| alphabet[*x as usize].clone()).collect()
}

#[inline]
fn gen_random_vec_u8(len: usize, gen_range: u8) -> Vec<u8> {
    (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u8>>()
}

#[inline]
fn gen_random_vec_u16(len: usize, gen_range: u16) -> Vec<u16> {
    (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u16>>()
}

#[inline]
fn gen_random_in_vec_u8_ref(vec: &mut Vec<u8>, len: usize, gen_range: u8) {
    *vec = (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u8>>()
}

#[inline]
fn gen_random_in_vec_u16_ref(vec: &mut Vec<u16>, len: usize, gen_range: u16) {
    *vec = (0..len)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..gen_range))
        .collect::<Vec<u16>>()
}

#[inline]
fn slice_into_string_from_char_alphabet(slice: &[u8], alphabet: &[char]) -> String {
    slice.iter().map(|x| alphabet[*x as usize]).collect()
}

#[inline]
fn slice_into_string_from_string_alphabet(slice: &[u16], alphabet: &[String]) -> String {
    slice.iter().map(|x| alphabet[*x as usize].clone()).collect()
}

#[inline]
fn get_bigrams_from_chars(mut chars: Chars) -> Vec<String> {
    (0..)
        .map(|_| chars.by_ref().take(L_BIGRAM).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>()
}

/// Vigenère cipher
/// (a) DISTORSION OF THE TEXT
pub fn vigenere_cipher_distortion(
    r: usize,
    l_grams: &[String],
    alphabet: &[char],
    l_little: u8,
) -> (Vec<String>, Vec<u16>) {
    let alphabet_len = alphabet.len();
    let (mut left, mut right) = (Vec::new(), Vec::new());
    let mut key = Vec::new();

    if l_little == 1 {
        let distore_n_grams =
            |vec: &mut Vec<String>, slice: &[String], key: &[u8], r: usize, alphabet_len: u8| {
                for l_gram in slice {
                    vec.push(slice_into_string_from_char_alphabet(
                        &l_gram
                            .chars()
                            .enumerate()
                            .map(|(i, c)| {
                                (*UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap() + key[i % r])
                                    % alphabet_len
                            })
                            .collect::<Vec<u8>>(),
                        alphabet,
                    ))
                }
            };

        let m = alphabet_len as u8;
        let key_local = gen_random_vec_u8(r, m);
        key = key_local.iter().map(|x| *x as u16).collect();
        rayon::join(
            || {
                distore_n_grams(
                    &mut left,
                    &l_grams[0..(l_grams.len() >> 1)],
                    &key_local,
                    r,
                    m,
                )
            },
            || {
                distore_n_grams(
                    &mut right,
                    &l_grams[(l_grams.len() >> 1)..],
                    &key_local,
                    r,
                    m,
                )
            },
        );
        left.extend_from_slice(&right);
    } else if l_little == 2 {
        let bigrams = make_n_gram_on_alphabet(L_BIGRAM, &UKR_ALPHABET);
        let distore_n_grams =
            |vec: &mut Vec<String>, slice: &[String], key: &[u16], r: usize, module: u16| {
                for l_gram in slice {
                    let chars = l_gram.chars();
                    vec.push(slice_into_string_from_string_alphabet(
                        &get_bigrams_from_chars(chars)
                            .iter()
                            .enumerate()
                            .map(|(i, str)| {
                                (*UKR_BIGRAM_REVERSE_MAP.get(str).unwrap() + key[i % r]) % module
                            })
                            .collect::<Vec<u16>>(),
                        &bigrams,
                    ));
                }
            };

        let m = (alphabet_len * alphabet_len) as u16;
        let key = gen_random_vec_u16(r, m);
        rayon::join(
            || distore_n_grams(&mut left, &l_grams[0..(l_grams.len() >> 1)], &key, r, m),
            || distore_n_grams(&mut right, &l_grams[(l_grams.len() >> 1)..], &key, r, m),
        );
        left.extend_from_slice(&right);
    }
    (left, key)
}

/// bigram affine substitution
/// (б) DISTORSION OF THE TEXT
pub fn gen_affine_distortion(
    l_grams: &[String],
    alphabet: &[char],
    l_little: u8,
) -> (Vec<String>, (u16, u16)) {
    let m = alphabet.len() as u16;
    let (mut a, mut b) = (0, 0);
    let (mut left, mut right) = (Vec::new(), Vec::new());

    if l_little == 1 {
        let distore_n_grams =
            |vec: &mut Vec<String>, slice: &[String], a: u16, b: u16, alphabet_len: u16| {
                for l_gram in slice {
                    vec.push(slice_into_string_from_char_alphabet(
                        &l_gram
                            .chars()
                            .map(|c| {
                                ((a * *UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap() as u16 + b)
                                    % alphabet_len) as u8
                            })
                            .collect::<Vec<u8>>(),
                        alphabet,
                    ))
                }
            };

        (a, b) = (thread_rng().gen_range(0..m), thread_rng().gen_range(0..m));
        rayon::join(
            || distore_n_grams(&mut left, &l_grams[0..(l_grams.len() >> 1)], a, b, m),
            || distore_n_grams(&mut right, &l_grams[(l_grams.len() >> 1)..], a, b, m),
        );
        left.extend_from_slice(&right);
    } else if l_little == 2 {
        let distore_n_grams = |distored_grams: &mut Vec<String>,
                               slice: &[String],
                               a: u16,
                               b: u16,
                               module: u16,
                               bigram_to_num_map: &HashMap<String, u16>,
                               num_to_bigram_map: &Vec<String>| {
            for l_gram in slice {
                let chars = l_gram.chars();
                let text_divided_on_bigrams = get_bigrams_from_chars(chars);
                distored_grams.push(
                    text_divided_on_bigrams
                        .iter()
                        .map(|str| {
                            num_to_bigram_map
                                [((a as u128 * *bigram_to_num_map.get(str).unwrap() as u128 +  b as u128 ) % module as u128) as usize]
                                .clone()
                        })
                        .collect(),
                )
            }
        };

        let mut module_squared = m * m;
        rayon::join(
            || a = thread_rng().gen_range(1..module_squared),
            || b = thread_rng().gen_range(1..module_squared),
        );
        let bigrams = make_n_gram_on_alphabet(L_BIGRAM, &UKR_ALPHABET);
        // aka bigram substitution
        let bigram_to_number_map: HashMap<String, u16> = {
            bigrams
                .iter()
                .map(|x| {
                    let mut char_indices = x.char_indices();
                    let (x_1, x_2) = (
                        *UKR_ALPHABET_REVERSE_MAP
                            .get(&char_indices.next().unwrap().1)
                            .unwrap(),
                        *UKR_ALPHABET_REVERSE_MAP
                            .get(&char_indices.next().unwrap().1)
                            .unwrap(),
                    );
                    (x.clone(), x_1 as u16 * m as u16 + x_2 as u16)
                })
                .collect()
        };

        rayon::join(
            || {
                distore_n_grams(
                    &mut left,
                    &l_grams[0..(l_grams.len() >> 1)],
                    a,
                    b,
                    module_squared,
                    &bigram_to_number_map,
                    &bigrams,
                )
            },
            || {
                distore_n_grams(
                    &mut right,
                    &l_grams[(l_grams.len() >> 1)..],
                    a,
                    b,
                    module_squared,
                    &bigram_to_number_map,
                    &bigrams,
                )
            },
        );
        left.extend_from_slice(&right);
    }
    (left, (a, b))
}

/// `generate_random_n_l_gram` -- generates `n` random `l`-grams
/// (в) DISTORSION OF THE TEXT
pub fn gen_random_n_l_grams(
    l: usize,
    n: usize,
    alphabet: &[char],
    l_little: u8,
) -> Vec<String> {
    let mut res = Vec::new();
    let alphabet_len = alphabet.len();

    if l_little == 1 {
        for _ in 0..n {
            res.push(gen_random_l_gram_char_alphabet(
                l,
                alphabet_len,
                alphabet,
            ));
        }
    } else if l_little == 2 {
        let bigrams = make_n_gram_on_alphabet(L_BIGRAM, &UKR_ALPHABET);
        for _ in 0..n  {
            res.push(gen_random_l_gram_string_alphabet(
                l>>1,
                bigrams.len(),
                &bigrams,
            ));
        }
    }

    res
}

/// `recurrent_generation_n_l_grams` -- generates `n` `l`-grams via recurrent formula
/// (г) DISTORSION OF THE TEXT
pub fn recurrent_generation_n_l_grams(
    l: usize,
    n: usize,
    alphabet: &[char],
    l_little: u8,
) -> Vec<String> {
    let m = alphabet.len();
    let mut res = Vec::new();

    if l_little == 1 {
        let m = m as u8;
        let (mut s_1, mut s_2) = (Vec::new(), Vec::new());
        rayon::join(
            || gen_random_in_vec_u8_ref(&mut s_1, l, m),
            || gen_random_in_vec_u8_ref(&mut s_2, l, m),
        );
        res.push(slice_u8_to_string(&s_1, alphabet));
        res.push(slice_u8_to_string(&s_2, alphabet));
        for _ in 2..n {
            let mut s_0 = Vec::new();
            let mut str = String::new();
            for i in 0..s_1.len() {
                let new_index = (s_1[i] + s_2[i]) % m;
                s_0.push(new_index);
                str.push(alphabet[new_index as usize]);
            }
            res.push(str);
            s_2 = s_1.clone();
            s_1 = s_0;
        }
    } else if l_little == 2 {
        let m = m as u16;
        let bigrams = make_n_gram_on_alphabet(L_BIGRAM, &UKR_ALPHABET);
        let m_squared = m * m;

        let (mut s_1, mut s_2) = (Vec::new(), Vec::new());
        rayon::join(
            || gen_random_in_vec_u16_ref(&mut s_1, l>>1, m_squared),
            || gen_random_in_vec_u16_ref(&mut s_2, l>>1, m_squared),
        );
        res.push(slice_u16_to_string(&s_1, &bigrams));
        res.push(slice_u16_to_string(&s_2, &bigrams));
        for _ in 2..n  {
            let mut s_0 = Vec::new();
            let mut str = String::new();
            for i in 0..s_1.len() {
                let new_index = (s_1[i] + s_2[i]) % m_squared;
                s_0.push(new_index);
                str += &bigrams[new_index as usize];
            }
            res.push(str);
            s_2 = s_1.clone();
            s_1 = s_0;
        }
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

pub fn make_frequency_table_for_long_chunks(
    content: &str,
    chunks: usize,
    range: Range<usize>,
) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    for i in range.clone() {
        // println!("{chunks} {i}");
        let mut new_content = content.to_string();
        for _ in 0..i {
            new_content.remove(0);
        }
        let sub_strings = make_n_gram_on_content_from_str(chunks, content);

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
    }
    // println!("for chunks {chunks} in range {:?} freq table DONE!!", range);
    map
}

pub fn make_frequency_table_custom_manual(content: &str, chunks: usize) -> HashMap<String, u64> {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    rayon::scope(|s| {
        s.spawn(|_s| map1 = make_frequency_table_for_long_chunks(content, chunks, 0..201));
        s.spawn(|_s| map2 = make_frequency_table_for_long_chunks(content, chunks, 301..501));
    });

    for x in map2 {
        match map1.get(&x.0) {
            None => {
                map1.insert(x.0.clone(), x.1);
            }
            Some(val) => {
                map1.insert(x.0.clone(), *val + x.1);
            }
        }
    }
    map1
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
                        None => 'a',
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
        // println!("acc: {acc}, val: {val}");
        if *val == 0. {
            acc
        } else {
            acc - *val * (*val).log2()
        }
    })
}

pub fn calculate_probs(h0: u64, h1: u64, total_l_grams_amount: usize) -> ((f64, f64), (f64, f64)) {
    // println!("total_l_grams: {total_l_grams_amount}");
    let all_cases = total_l_grams_amount as f64;
    let (p_h_0, p_h_1) = (
        h0 as f64 / all_cases,
        h1 as f64 / all_cases, // if h0 == 0 { 0. } else { h0 as f64 / all_cases },
        // if h1 == 0 { 0. } else { h1 as f64 / all_cases },
    );
    let p_h_0_1 = p_h_0 * p_h_1;
    // println!("p_h_0_1: {p_h_0_1}, p_h_0: {p_h_0}, p_h_1:{p_h_1}");
    (
        (p_h_0, p_h_1),
        (p_h_0_1 / p_h_0, p_h_0_1 / p_h_1), // if p_h_0 == 0. { 0. } else { p_h_0_1 / p_h_0 },
        // if p_h_1 == 0. { 0. } else { p_h_0_1 / p_h_1 },
    )
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

pub fn divide_into_l_grams(
    n_gram_l1: &mut Vec<String>,
    n_gram_l2: &mut Vec<String>,
    n_gram_l3: &mut Vec<String>,
    n_gram_l4: &mut Vec<String>,
    content: &String,
) {
    rayon::scope(|s| {
        s.spawn(|_s| {
            *n_gram_l1 = make_n_gram_on_content_from_str(L1, &content);
        });
        s.spawn(|_s| {
            *n_gram_l2 = make_n_gram_on_content_from_str(L2, &content);
        });
        s.spawn(|_s| {
            *n_gram_l3 = make_n_gram_on_content_from_str(L3, &content);
        });
        s.spawn(|_s| {
            *n_gram_l4 = make_n_gram_on_content_from_str(L4, &content);
        });
    });
}

pub fn double_content(str: &String) -> String {
    let mut str1 = str.clone();
    let mut str2 = str.clone();
    str2.remove(0);
    str1 + str2.as_str()
}

#[test]
fn double_content_test() {
    let str = "іванбагрянийтигроловичастинапершарозділпершийдраконвирячившивогненніочідихаючиполумямідимомпотрясаючиревомпустеліінетраівогненнимхвостомзамітаючислідлетівдраконнезкитайськихказокінезпагодтібетувінзнявсядесьзгромохкогоцентрукраїничудесвилетівзчорногопеклаземлілюдоловівігнавнад".to_string();
    let doubled_content = double_content(&str);

    println!("doubled_content: {:?}", doubled_content)
}

#[test]
fn generate_random_n_gram_test() {
    let rand_gen_n_gram_1 = gen_random_n_l_grams(100, 10, &UKR_ALPHABET, 1);
    let rand_gen_n_gram_2 = gen_random_n_l_grams(100, 10, &UKR_ALPHABET, 2);

    println!("random n_gram_1: {:?}, \n\t\trandom n_gram_2: {:?}", rand_gen_n_gram_1, rand_gen_n_gram_2)
}

#[test]
fn recurrent_gen_test() {
    let recur_1 = recurrent_generation_n_l_grams(10, 10, &UKR_ALPHABET, 1);
    let recur_2 = recurrent_generation_n_l_grams(10, 10, &UKR_ALPHABET, 2);

    println!("recurrent n_gram_1: {:?}, \n\t\trecurrent n_gram_2: {:?}", recur_1, recur_2)
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
    let affine_distortion_1 = gen_affine_distortion(&n_grams[0..10], &UKR_ALPHABET, 1);
    // let affine_distortion_2 = gen_affine_distortion(&n_grams[0..10], &UKR_ALPHABET, 1);
    let affine_distortion_2 = gen_affine_distortion(&n_grams[0..10], &UKR_ALPHABET, 2);
    println!(
        "original: {:?} \n\t\t affine distorted l=1 n_gram: {:?} \n\t\t{:?} \n\t  affine distorted l=2 n_gram: {:?} \n\t\t{:?}",
        &n_grams[0..10],
        affine_distortion_1,
        &n_grams[0..10]
            .iter()
            .map(|c| c
                .chars()
                .map(|c| *UKR_ALPHABET_REVERSE_MAP.get(&c).unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>(),
        affine_distortion_2,
        &n_grams[0..10]
            .iter()
            .map(|c|
                get_bigrams_from_chars(c
                    .chars()).iter()
                    .map(|str| *UKR_BIGRAM_REVERSE_MAP.get(str).unwrap())
                    .collect::<Vec<u16>>())
            .collect::<Vec<Vec<u16>>>()
    )
}

#[test]
fn generate_bigram_affine_distortion_test() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let chunks = 2;
    let n_grams = make_n_gram_on_file_content(&filepath, chunks);
    let bigram_affine_distortion_1 = gen_affine_distortion(&n_grams[0..10], &UKR_ALPHABET, 2);
    println!(
        "original: {:?} \n\t\t affine distorted n_gram: {:?} \n\t\t{:?}",
        &n_grams[0..10],
        bigram_affine_distortion_1,
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
    let vigenere_cipher_distortion = vigenere_cipher_distortion(1, &n_grams[0..10], &UKR_ALPHABET, 1);
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

#[test]
fn make_n_gram_alphabet_test() {
    let n = 2;
    println!("{n}_grams: {:?}", make_n_gram_on_alphabet(n, &UKR_ALPHABET))
}
