use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use rand::{thread_rng, Rng};

use regex::Regex;
use crate::UKR_ALPHABET;

#[inline]
fn generate_random_l_gram(l: usize, alphabet_len: usize, alphabet: &[char]) -> String {
    (0..l).map(|x| alphabet[thread_rng().gen_range(0..alphabet_len)]).collect()
}

#[inline]
fn slice_u8_to_string(slice: &[u8], alphabet: &[char]) -> String {
    slice.iter().map(|x| alphabet[*x as usize]).collect()
}

fn recurrent_generation_n_l_gram(acc: usize, l: usize, n: usize, s_1: &[u8], s_2: &[u8], alphabet_len: usize, alphabet: &[char], res: &mut Vec<String>) {
    if acc < n {
        let (s_0, str) = (0..s_1.len()).into_iter().map(|i| {
            let new_index = (s_1[i] + s_2[i]) % alphabet_len;
            (new_index, alphabet[new_index as usize])
        }).collect::<(Vec<u8>, String)>();
        res.push(str);
        recurrent_generation_n_l_gram(acc + 1, l, n, &s_0, s_1, alphabet_len, alphabet, res)
    }
}

/// `generate_random_n_l_gram` -- generates `n` random `l`-grams
/// (в) DISTORSION OF THE TEXT
pub fn generate_random_n_l_grams(l: usize, acc: usize, n: usize, alphabet: &[char]) -> Vec<String> {
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
    let alphabet_len = alphabet.len();
    let mut res = Vec::new();
    let s_1 = (0..l).thread_rng().gen_range(0..alphabet_len).collect::<Vec<u8>>();
    let s_2 = (0..l).thread_rng().gen_range(0..alphabet_len).collect::<Vec<u8>>();
    res.push(slice_u8_to_string(&s_1, alphabet));
    res.push(slice_u8_to_string(&s_2, alphabet));
    recurrent_generation_n_l_gram(3, l, n, &s_1, &s_2, alphabet_len, alphabet, &mut res);

    res
}


///
pub fn make_frequency_table(
    filepath: &str,
    chunks: usize,
) -> HashMap<String, u64> {
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
            Some(val) => { map.insert(x.clone(), *val + 1); }
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
        .take_while(|s| {
            !s.is_empty()
        })
        .collect::<Vec<_>>()
}

pub fn is_n_gram_forbidden(n_gram: &String, frequency_table: &HashMap<String, u64>, threshold: u64) -> bool {
    match frequency_table.get(n_gram) {
        None => { true }
        Some(val) => {
            if *val < threshold {
                return true;
            }
            false
        }
    }
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
        if *val == 0. { acc } else { acc - *val * (*val).log2() }
    })
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
    let rand_gen_n_gram = generate_random_n_l_grams(10, 10, &UKR_ALPHABET);

    println!("random n_gram: {:?}", rand_gen_n_gram)
}