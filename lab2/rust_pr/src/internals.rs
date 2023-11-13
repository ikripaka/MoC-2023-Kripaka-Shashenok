use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use regex::Regex;

pub(super) fn make_frequency_table(filepath: &str, alphabet: Vec<String>) -> HashMap<String, u64> {
    let map = HashMap::new();
    let file = File::open(filepath).unwrap();




    map
}

pub(super) fn make_n_gram() -> HashMap<String, u64> {

    todo!()
}

pub(super) fn calculate_entropy() -> HashMap<String, u64> {
    todo!()
}
pub(super) fn make_probability_table() -> HashMap<String, u64> {
    todo!()
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
