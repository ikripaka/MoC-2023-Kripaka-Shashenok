use crate::internals::{bigram_affine_distortion, calculate_probs, double_content, generate_affine_distortion, generate_random_n_l_grams, make_frequency_table, make_frequency_table_from_file, make_n_gram_on_content_from_str, make_n_gram_on_file_content, recurrent_generation_n_l_grams, vigenere_cipher_distortion};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};
use chrono::Local;
use dotenv::dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let j = 200;
    let j_bigram = 200;
    let k_empty = 100;

    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    content = double_content(&mut content);

    // #0
    let (
        mut res1_0,
        mut res1_1_r1,
        mut res1_1_r2,
        mut res1_1_r3,
        mut res1_2,
        mut res1_3,
        mut res1_4,
    ) = Default::default();
    let (
        mut res2_0,
        mut res2_1_r1,
        mut res2_1_r2,
        mut res2_1_r3,
        mut res2_2,
        mut res2_3,
        mut res2_4,
    ) = Default::default();

    let (
        mut res3_0,
        mut res3_1_r1,
        mut res3_1_r2,
        mut res3_1_r3,
        mut res3_2,
        mut res3_3,
        mut res3_4,
    ) = Default::default();
    let (
        mut res4_0,
        mut res4_1_r1,
        mut res4_1_r2,
        mut res4_1_r3,
        mut res4_2,
        mut res4_3,
        mut res4_4,
    ) = Default::default();

    // calculating frequency tables for text
    let (
        mut freq_table_bigram,
        mut freq_table_l1,
        mut freq_table_l2,
        mut freq_table_l3,
        mut freq_table_l4,
    ) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            freq_table_bigram = make_frequency_table(&content, L_BIGRAM);
        });
        s.spawn(|_s| {
            freq_table_l1 = make_frequency_table(&content, L1);
        });
        s.spawn(|_s| {
            freq_table_l2 = make_frequency_table(&content, L2);
        });
        s.spawn(|_s| {
            freq_table_l3 = make_frequency_table(&content, L3);
        });
        s.spawn(|_s| {
            freq_table_l4 = make_frequency_table(&content, L4);
        });
    });
    println!("Frequency tables are calculated (criterion_5)");

    let (mut bigrams, mut three_grams, mut n_gram_l1, mut n_gram_l2, mut n_gram_l3, mut n_gram_l4): (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ) = (vec![], vec![], vec![], vec![], vec![], vec![]);
    rayon::scope(|s| {
        s.spawn(|_s| {
            bigrams = make_n_gram_on_content_from_str(L_BIGRAM, &content);
        });
        s.spawn(|_s| {
            three_grams = make_n_gram_on_content_from_str(L_THREE_GRAM, &content);
        });
        s.spawn(|_s| {
            n_gram_l1 = make_n_gram_on_content_from_str(L1, &content);
        });
        s.spawn(|_s| {
            n_gram_l2 = make_n_gram_on_content_from_str(L2, &content);
        });
        s.spawn(|_s| {
            n_gram_l3 = make_n_gram_on_content_from_str(L3, &content);
        });
        s.spawn(|_s| {
            n_gram_l4 = make_n_gram_on_content_from_str(L4, &content);
        });
    });
    println!("N grams are made (criterion_5)");

    let (
        mut distorted_n_grams_l1_1_r1,
        mut distorted_n_grams_l1_1_r2,
        mut distorted_n_grams_l1_1_r3,
        mut distorted_n_grams_l1_2,
        mut distorted_n_grams_l1_3,
        mut distorted_n_grams_l1_4,
    ) = Default::default();
    let (
        mut distorted_n_grams_l2_1_r1,
        mut distorted_n_grams_l2_1_r2,
        mut distorted_n_grams_l2_1_r3,
        mut distorted_n_grams_l2_2,
        mut distorted_n_grams_l2_3,
        mut distorted_n_grams_l2_4,
    ) = Default::default();
    let (
        mut distorted_n_grams_l3_1_r1,
        mut distorted_n_grams_l3_1_r2,
        mut distorted_n_grams_l3_1_r3,
        mut distorted_n_grams_l3_2,
        mut distorted_n_grams_l3_3,
        mut distorted_n_grams_l3_4,
    ) = Default::default();
    let (
        mut distorted_n_grams_l4_1_r1,
        mut distorted_n_grams_l4_1_r2,
        mut distorted_n_grams_l4_1_r3,
        mut distorted_n_grams_l4_2,
        mut distorted_n_grams_l4_3,
        mut distorted_n_grams_l4_4,
    ) = Default::default();

    rayon::scope(|s| {
        s.spawn(|_s| {
            distorted_n_grams_l1_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l1_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l1_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            // n_gram_l1.truncate(N1);
            // distorted_n_grams_l1_2 = bigram_affine_distortion(&n_gram_l1, &UKR_ALPHABET);
            distorted_n_grams_l1_2 = generate_affine_distortion(L1, &n_gram_l1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l1_3 = generate_random_n_l_grams(L1, N1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l1_4 = recurrent_generation_n_l_grams(L1, N1, &UKR_ALPHABET);
        });

        s.spawn(|_s| {
            distorted_n_grams_l2_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l2, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l2_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l2, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l2_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l2, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            // distorted_n_grams_l2_2 = bigram_affine_distortion(&n_gram_l2, &UKR_ALPHABET);
            distorted_n_grams_l2_2 = generate_affine_distortion(L2, &n_gram_l2, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l2_3 = generate_random_n_l_grams(L2, N1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l2_4 = recurrent_generation_n_l_grams(L2, N1, &UKR_ALPHABET);
        });

        s.spawn(|_s| {
            distorted_n_grams_l3_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l3, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l3_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l3, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l3_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l3, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            // distorted_n_grams_l3_2 = bigram_affine_distortion(&n_gram_l3, &UKR_ALPHABET);
            distorted_n_grams_l3_2 = generate_affine_distortion(L3, &n_gram_l3, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l3_3 = generate_random_n_l_grams(L3, N1, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l3_4 = recurrent_generation_n_l_grams(L3, N1, &UKR_ALPHABET);
        });

        s.spawn(|_s| {
            distorted_n_grams_l4_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l4, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l4_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l4, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l4_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l4, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            // distorted_n_grams_l4_2 = bigram_affine_distortion(&n_gram_l4, &UKR_ALPHABET);
            distorted_n_grams_l4_2 = generate_affine_distortion(L4, &n_gram_l4, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l4_3 = generate_random_n_l_grams(L4, N2, &UKR_ALPHABET);
        });
        s.spawn(|_s| {
            distorted_n_grams_l4_4 = recurrent_generation_n_l_grams(L4, N2, &UKR_ALPHABET);
        });
    });
    println!("Distorted N grams are made (criterion_5)");

    rayon::scope(|s| {
        s.spawn(|_s| {
            res1_0 = criterion_5(&freq_table_l1, &freq_table_bigram, &n_gram_l1, j, j_bigram, k_empty);
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_1_r1 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_1_r1.0, j, j_bigram, k_empty);
            println!(
                "res1_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_1_r2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_1_r2.0, j, j_bigram, k_empty);
            println!(
                "res1_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_1_r3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_1_r3.0, j, j_bigram, k_empty);
            println!(
                "res1_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_2.0, j, j_bigram, k_empty);
            println!(
                "res1_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_3, j, j_bigram, k_empty);
            println!(
                "res1_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res1_4 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l1_4, j, j_bigram, k_empty);
            println!(
                "res1_4 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });

        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_0 = criterion_5(&freq_table_l1, &freq_table_bigram, &n_gram_l2, j, j_bigram, k_empty);
            println!(
                "res2_0 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_1_r1 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_1_r1.0, j, j_bigram, k_empty);
            println!(
                "res2_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_1_r2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_1_r2.0, j, j_bigram, k_empty);
            println!(
                "res2_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_1_r3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_1_r3.0, j, j_bigram, k_empty);
            println!(
                "res2_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_2.0, j, j_bigram, k_empty);
            println!(
                "res2_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_3, j, j_bigram, k_empty);
            println!(
                "res2_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            )
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res2_4 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l2_4, j, j_bigram, k_empty);
            println!(
                "res2_4 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });

        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_0 = criterion_5(&freq_table_l1, &freq_table_bigram, &n_gram_l3, j, j_bigram, k_empty);
            println!(
                "res3_0 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r1 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_1_r1.0, j, j_bigram, k_empty);
            println!(
                "res3_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_1_r2.0, j, j_bigram, k_empty);
            println!(
                "res3_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_1_r3.0, j, j_bigram, k_empty);
            println!(
                "res3_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_2.0, j, j_bigram, k_empty);
            println!(
                "res3_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_3, j, j_bigram, k_empty);
            println!(
                "res3_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_4 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l3_4, j, j_bigram, k_empty);
            println!(
                "res3_4 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });

        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_0 = criterion_5(&freq_table_l1, &freq_table_bigram, &n_gram_l4, j, j_bigram, k_empty);
            println!(
                "res4_0 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r1 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_1_r1.0, j, j_bigram, k_empty);
            println!(
                "res4_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_1_r2.0, j, j_bigram, k_empty);
            println!(
                "res4_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_1_r3.0, j, j_bigram, k_empty);
            println!(
                "res4_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_2 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_2.0, j, j_bigram, k_empty);
            println!(
                "res4_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_3 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_3, j, j_bigram, k_empty);
            println!(
                "res4_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_4 = criterion_5(&freq_table_l1, &freq_table_bigram, &distorted_n_grams_l4_4, j, j_bigram, k_empty);
            println!(
                "res4_4 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
    });
    println!(
        "IT FINALLY FINISHED!! Time:{}",
        (Local::now() - time_prev).num_minutes()
    );

    println!(
        "Result: \

        \n\t (criterion_5) [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_5) [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} , \
        \n\t (criterion_5) [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_5) [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (criterion_5) [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_5) [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_5) [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

                ",
        res1_0,
        calculate_probs(res1_0.0, res1_0.1, n_gram_l1.len()),
        res1_1_r1,
        calculate_probs(res1_1_r1.0, res1_1_r1.1, distorted_n_grams_l1_1_r1.0.len()),
        res1_1_r2,
        calculate_probs(res1_1_r2.0, res1_1_r2.1, distorted_n_grams_l1_1_r2.0.len()),
        res1_1_r3,
        calculate_probs(res1_1_r3.0, res1_1_r3.1, distorted_n_grams_l1_1_r3.0.len()),
        res1_2,
        calculate_probs(res1_2.0, res1_2.1, distorted_n_grams_l1_2.0.len()),
        res1_3,
        calculate_probs(res1_3.0, res1_3.1, distorted_n_grams_l1_3.len()),
        res1_4,
        calculate_probs(res1_4.0, res1_4.1, distorted_n_grams_l1_4.len()),
        res2_0,
        calculate_probs(res2_0.0, res2_0.1, n_gram_l2.len()),
        res2_1_r1,
        calculate_probs(res2_1_r1.0, res2_1_r1.1, distorted_n_grams_l2_1_r1.0.len()),
        res2_1_r2,
        calculate_probs(res2_1_r2.0, res2_1_r2.1, distorted_n_grams_l2_1_r2.0.len()),
        res2_1_r3,
        calculate_probs(res2_1_r3.0, res2_1_r3.1, distorted_n_grams_l2_1_r3.0.len()),
        res2_2,
        calculate_probs(res2_2.0, res2_2.1, distorted_n_grams_l2_2.0.len()),
        res2_3,
        calculate_probs(res2_3.0, res2_3.1, distorted_n_grams_l2_3.len()),
        res2_4,
        calculate_probs(res2_4.0, res2_4.1, distorted_n_grams_l2_4.len()),
        res3_0,
        calculate_probs(res3_0.0, res3_0.1, n_gram_l3.len()),
        res3_1_r1,
        calculate_probs(res3_1_r1.0, res3_1_r1.1, distorted_n_grams_l3_1_r1.0.len()),
        res3_1_r2,
        calculate_probs(res3_1_r2.0, res3_1_r2.1, distorted_n_grams_l3_1_r2.0.len()),
        res3_1_r3,
        calculate_probs(res3_1_r3.0, res3_1_r3.1, distorted_n_grams_l3_1_r3.0.len()),
        res3_2,
        calculate_probs(res3_2.0, res3_2.1, distorted_n_grams_l3_2.0.len()),
        res3_3,
        calculate_probs(res3_3.0, res3_3.1, distorted_n_grams_l3_3.len()),
        res3_4,
        calculate_probs(res3_4.0, res3_4.1, distorted_n_grams_l3_4.len()),
        res4_0,
        calculate_probs(res4_0.0, res4_0.1, n_gram_l4.len()),
        res4_1_r1,
        calculate_probs(res4_1_r1.0, res4_1_r1.1, distorted_n_grams_l4_1_r1.0.len()),
        res4_1_r2,
        calculate_probs(res4_1_r2.0, res4_1_r2.1, distorted_n_grams_l4_1_r2.0.len()),
        res4_1_r3,
        calculate_probs(res4_1_r3.0, res4_1_r3.1, distorted_n_grams_l4_1_r3.0.len()),
        res4_2,
        calculate_probs(res4_2.0, res4_2.1, distorted_n_grams_l4_2.0.len()),
        res4_3,
        calculate_probs(res4_3.0, res4_3.1, distorted_n_grams_l4_3.len()),
        res4_4,
        calculate_probs(res4_4.0, res4_4.1, distorted_n_grams_l4_4.len()),
    )
}

fn criterion_5(
    l_gram_frequency_table: &HashMap<String, u64>,
    bigram_frequency_table: &HashMap<String, u64>,
    l_grams: &Vec<String>,
    j: usize,
    j_bigram: usize,
    k_empty: usize,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    let mut l_grams_b_prh = sort_hash_map(l_gram_frequency_table);
    l_grams_b_prh.truncate(j);
    let mut bigrams_b_prh = sort_hash_map(bigram_frequency_table);
    bigrams_b_prh.truncate(j_bigram);


    for l_gram in l_grams {
        let (mut bigram_crates, mut l_gram_crates): (Vec<usize>, Vec<usize>) =
            (vec![0; j_bigram], vec![0; j]);
        rayon::scope(|s| {
            s.spawn(|_s| {
                for (i, prh_bigram) in bigrams_b_prh.iter().enumerate() {
                    if l_gram.contains(&prh_bigram.0) {
                        bigram_crates[i] += 1;
                    }
                }
            });
            // s.spawn(|_s| {
            //     for (i, prh_bigram) in l_grams_b_prh.iter().enumerate() {
            //         if l_gram.contains(&prh_bigram.0) {
            //             l_gram_crates[i] += 1;
            //         }
            //     }
            // });
        });


        let f_empty = {
            let empty1 = bigram_crates.iter().fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            // let empty2 = l_gram_crates.iter().fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            empty1
        };
        // println!("f_empty: {}, k_empty: {}", f_empty, k_empty);
        if f_empty <= k_empty {
            h_1 += 1;
        } else {
            h_0 += 1;
        }
    }
    (h_0, h_1)
}

fn criterion_5_log(
    l_gram_frequency_table: &HashMap<String, u64>,
    bigram_frequency_table: &HashMap<String, u64>,
    l_grams: &Vec<String>,
    j: usize,
    j_bigram: usize,
    k_empty: usize,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    let mut l_grams_b_prh = sort_hash_map(l_gram_frequency_table);
    l_grams_b_prh.truncate(j);
    let mut bigrams_b_prh = sort_hash_map(bigram_frequency_table);
    bigrams_b_prh.truncate(j_bigram);


    for l_gram in l_grams {
        let (mut bigram_crates, mut l_gram_crates): (Vec<usize>, Vec<usize>) =
            (vec![0; j_bigram], vec![0; j]);
        rayon::scope(|s| {
            s.spawn(|_s| {
                for (i, prh_bigram) in bigrams_b_prh.iter().enumerate() {
                    if l_gram.contains(&prh_bigram.0) {
                        bigram_crates[i] += 1;
                    }
                }
            });
            s.spawn(|_s| {
                for (i, prh_bigram) in l_grams_b_prh.iter().enumerate() {
                    if l_gram.contains(&prh_bigram.0) {
                        l_gram_crates[i] += 1;
                    }
                }
            });
        });


        let f_empty = {
            let empty1 = bigram_crates.iter().fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            let empty2 = l_gram_crates.iter().fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            println!("{:?} {:?} {} {}",bigram_crates, l_gram_crates,empty1, empty2);
            empty1 + empty2
        };
        // println!("f_empty: {}, k_empty: {}", f_empty, k_empty);
        if f_empty <= k_empty {
            h_1 += 1;
        } else {
            h_0 += 1;
        }
    }
    (h_0, h_1)
}

fn sort_hash_map(map: &HashMap<String, u64>) -> Vec<(String, u64)> {
    let mut vec: Vec<(String, u64)> = map.iter().map(|(x, y)| (x.clone(), *y)).collect();
    vec.sort_by(|x, y| x.1.cmp(&y.1));
    vec
}

#[test]
fn sorting() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let (chunks, threshold) = (2, 10);
    let freq_table = make_frequency_table_from_file(&filepath, chunks);
    println!("{:?}", sort_hash_map(&freq_table))
}