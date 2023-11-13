use dotenv::dotenv;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use rust_pr::criterions;


const UKR_ALPHABET: [char; 33] = [
    'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];

fn main() {
    dotenv().ok();

    // format_file(
    //     std::env::var("SOURCE_FILENAME").unwrap().as_mut_str(),
    //     std::env::var("OUTPUT_FILENAME").unwrap().as_mut_str(),
    // );

    criterions::criterion_1_0::run();
    criterions::criterion_1_1::run();
    criterions::criterion_1_2::run();
    criterions::criterion_1_3::run();
    criterions::criterion_4_0::run();
    criterions::criterion_5_0::run();

    println!("{}", "ALL IS OK");
}
