use dotenv::dotenv;
use regex::Regex;
use rust_pr::internals::{calculate_entropy, format_file, is_n_gram_forbidden, make_frequency_table, make_n_gram_on_alphabet, make_n_gram_on_file_content, make_probability_table};
use rust_pr::{criterions, L1, L2, L3, L4, N1, N2, THRESHOLD, UKR_ALPHABET};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

fn main() {
    dotenv().ok();
    format_file(
        std::env::var("SOURCE_FILENAME").unwrap().as_mut_str(),
        std::env::var("OUTPUT_FILENAME").unwrap().as_mut_str(),
    );
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();

    // ====== PROGRAM STARTED ======

    let (mut l1_gram, mut l2_gram, mut l3_gram, mut l4_gram) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());

    rayon::scope(|s| {
        s.spawn(|_s| {
            l1_gram = make_n_gram_on_file_content(&filepath, L1);
        });
        s.spawn(|_s| {
            l2_gram = make_n_gram_on_file_content(&filepath, L2);
        });
        s.spawn(|_s| {
            l3_gram = make_n_gram_on_file_content(&filepath, L3);
        });
        s.spawn(|_s| {
            l4_gram = make_n_gram_on_file_content(&filepath, L4);
        });
    });
    println!(
        "{}, {}, {}, {}",
        l1_gram.len(),
        l2_gram.len(),
        l3_gram.len(),
        l4_gram.len()
    );

    // let chunks = 2;
    // let monogram = make_n_gram_on_alphabet(chunks, &UKR_ALPHABET);
    // let freq_table1 = make_frequency_table(&filepath, &monogram, chunks);
    // println!("monogram entropy: {}", calculate_entropy(&make_probability_table(&freq_table1)));

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


#[test]
fn make_n_gram_on_file_content_test() {
    dotenv().ok();
    let chunks = L3;
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let n_gram = make_n_gram_on_file_content( &filepath, chunks);
    // println!("{:?}", n_gram)
    assert!({
        let mut res = true;
        for chunk in n_gram {
            if chunk.chars().count() != chunks {
                res = false
            }
        }
        res
    })
}

#[test]
fn forbidden_n_gram_test() {
    dotenv().ok();
    let chunks1 = 2;
    let chunks2 = L1;
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let frequency_table1 =  make_frequency_table(&filepath, chunks1);
    let frequency_table2 =  make_frequency_table(&filepath, chunks2);
    // println!("{:?}", frequency_table1);
    // println!("{:?}", frequency_table2);
    assert!(is_n_gram_forbidden(&"аааааааааа".to_string(), &frequency_table2, THRESHOLD));
    assert!(is_n_gram_forbidden(&"одинголосс".to_string(), &frequency_table2, THRESHOLD));
    assert!(is_n_gram_forbidden(&"кривиласят".to_string(), &frequency_table2, THRESHOLD));

    assert!(!is_n_gram_forbidden(&"ча".to_string(), &frequency_table1, THRESHOLD));
    assert!(!is_n_gram_forbidden(&"ми".to_string(), &frequency_table1, THRESHOLD));
    assert!(!is_n_gram_forbidden(&"ре".to_string(), &frequency_table1, THRESHOLD));
}

#[test]
fn frequency_table_len_test() {
    dotenv().ok();
    format_file(
        std::env::var("SOURCE_FILENAME").unwrap().as_mut_str(),
        std::env::var("OUTPUT_FILENAME").unwrap().as_mut_str(),
    );
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let n_gram1 =  make_n_gram_on_file_content(&filepath, L1);
    let n_gram2 =  make_n_gram_on_file_content(&filepath, L2);
    let n_gram3 =  make_n_gram_on_file_content(&filepath, L3);
    let n_gram4 =  make_n_gram_on_file_content(&filepath, L4);
    println!("10:{}, 100:{}, 1000:{}, 10_000:{}", n_gram1.len(),n_gram2.len(),n_gram3.len(),n_gram4.len(),);

    assert!(n_gram1.len() > N1);
    assert!(n_gram2.len() > N1);
    assert!(n_gram3.len() > N1);
    assert!(n_gram4.len() > N2);
}

