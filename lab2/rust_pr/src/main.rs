use dotenv::dotenv;

use rust_pr::{criterions, L1, L2, L3, L4, N1, N2, THRESHOLD};
use rust_pr::internals::{
    format_file, is_n_gram_prohibited_with_ngrams,
    make_frequency_table_from_file, make_n_gram_on_file_content,
};

fn main() {
    dotenv().ok();
    // format_file(
    //     std::env::var("SOURCE_FILENAME").unwrap().as_mut_str(),
    //     std::env::var("OUTPUT_FILENAME").unwrap().as_mut_str(),
    // );
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();

    // ====== PROGRAM STARTED ======

    // criterions::criterion_1_0::run(&filepath);
    // criterions::criterion_1_1::run(&filepath);
    // criterions::criterion_1_2::run(&filepath);
    // criterions::criterion_1_3::run(&filepath);
    criterions::criterion_3_0::run(&filepath);
    // criterions::criterion_4_0::run(&filepath);
    // criterions::criterion_5_0::run(&filepath);
    // criterions::criterion_5_1::run(&filepath);
    // criterions::struct_deflate::run(&filepath);
    // criterions::struct_bwt::run(&filepath);
    // criterions::struct_lzma::run(&filepath);

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
    let n_gram = make_n_gram_on_file_content(&filepath, chunks);
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
    let frequency_table1 = make_frequency_table_from_file(&filepath, chunks1);
    let frequency_table2 = make_frequency_table_from_file(&filepath, chunks2);
    // println!("{:?}", frequency_table1);
    // println!("{:?}", frequency_table2);
    assert!(is_n_gram_prohibited_with_ngrams(
        &"аааааааааа".to_string(),
        &frequency_table2,
        THRESHOLD,
    ));
    assert!(is_n_gram_prohibited_with_ngrams(
        &"одинголосс".to_string(),
        &frequency_table2,
        THRESHOLD,
    ));
    assert!(is_n_gram_prohibited_with_ngrams(
        &"кривиласят".to_string(),
        &frequency_table2,
        THRESHOLD,
    ));

    assert!(!is_n_gram_prohibited_with_ngrams(
        &"ча".to_string(),
        &frequency_table1,
        THRESHOLD,
    ));
    assert!(!is_n_gram_prohibited_with_ngrams(
        &"ми".to_string(),
        &frequency_table1,
        THRESHOLD,
    ));
    assert!(!is_n_gram_prohibited_with_ngrams(
        &"ре".to_string(),
        &frequency_table1,
        THRESHOLD,
    ));
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
    let n_gram1 = make_n_gram_on_file_content(&filepath, L1);
    let n_gram2 = make_n_gram_on_file_content(&filepath, L2);
    let n_gram3 = make_n_gram_on_file_content(&filepath, L3);
    let n_gram4 = make_n_gram_on_file_content(&filepath, L4);
    println!(
        "10:{}, 100:{}, 1000:{}, 10_000:{}",
        n_gram1.len(),
        n_gram2.len(),
        n_gram3.len(),
        n_gram4.len(),
    );

    assert!(n_gram1.len() > N1);
    assert!(n_gram2.len() > N1);
    assert!(n_gram3.len() > N1);
    assert!(n_gram4.len() > N2);
}
