use dotenv::dotenv;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use rust_pr::criterions;
use rust_pr::internals::{calculate_entropy, make_frequency_table, make_n_gram, make_probability_table};


const UKR_ALPHABET: [char; 33] = [
    'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];

fn main() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME").unwrap().as_str().to_string();

    // format_file(
    //     std::env::var("SOURCE_FILENAME").unwrap().as_mut_str(),
    //     std::env::var("OUTPUT_FILENAME").unwrap().as_mut_str(),
    // );

    let chunks = 2;
    let monogram = make_n_gram(chunks, &UKR_ALPHABET);
    let freq_table1 = make_frequency_table(&filepath, &monogram, chunks);
    println!("monogram entropy: {}", calculate_entropy(&make_probability_table(&freq_table1)));

    // let bigram = make_n_gram(2,&UKR_ALPHABET);
    // let frequency_table = make_frequency_table(&filepath, &bigram , 2);


    // println!("bigram: {:?} \n\t\t frequency table: {:?}",bigram.len(), frequency_table.len());


    criterions::criterion_1_0::run();
    criterions::criterion_1_1::run();
    criterions::criterion_1_2::run();
    criterions::criterion_1_3::run();
    criterions::criterion_4_0::run();
    criterions::criterion_5_0::run();

    println!("{}", "ALL IS OK");
}
