use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use regex::Regex;

pub fn make_frequency_table(
    filepath: &str,
    alphabet: &Vec<String>,
    chunks: usize,
) -> HashMap<String, u64> {
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);

    //trim string to specific size
    if content.len() % chunks != 0 {
        for _ in (0..(content.len() % chunks)) {
            content.remove(content.len() - 1);
        }
    }

    let mut chars = content.chars();
    let mut sub_string = (0..)
        .map(|_| chars.by_ref().take(chunks).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    sub_string.remove(sub_string.len() - 1);

    let mut map = alphabet
        .iter()
        .map(|x| (x.clone(), 0))
        .collect::<HashMap<String, u64>>();

    println!("substr {:?}", sub_string.len());

    for x in sub_string {
        match map.get(&x) {
            None => {
                println!("{x} isn't in map, \n{:?}", 5)
            }
            Some(_) => {}
        }
        map.insert(x.clone(), map.get(&x).unwrap() + 1);
    }
    map
}

pub fn make_n_gram(n: usize, alphabet: &[char]) -> Vec<String> {
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
       if *val == 0. {acc} else { acc - *val * (*val).log2() }})
}

/// **format_file** -- formats file and deletes all redundant symbol
/// except:
///     * ukrainian alphabet
///     * "ґ" replaced into -> "г"
fn format_file(in_filepath: &str, out_filepath: &str) {
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
