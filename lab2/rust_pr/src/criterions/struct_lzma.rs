use std::fs::File;
use std::io::{Read, Write};

use chrono::Local;
use dotenv::dotenv;

use crate::{L1, L2, L3, L4, L_BIGRAM, L_THREE_GRAM, N1, R1, R2, R3, UKR_ALPHABET};
use crate::internals::{calculate_probs, double_content, gen_affine_distortion, gen_random_l_gram_char_alphabet, gen_random_n_l_grams, make_frequency_table, make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, vigenere_cipher_distortion};

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let threshold = 0.2;

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
    println!("Frequency tables are calculated (struct_lzma)");

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
    println!("N grams are made (struct_lzma)");

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


    for l_little in 1..=2 {
        rayon::scope(|s| {
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                // n_gram_l1.truncate(N1);
                // distorted_n_grams_l1_2 = bigram_affine_distortion(&n_gram_l1, &UKR_ALPHABET);
                distorted_n_grams_l1_2 = gen_affine_distortion(&n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_3 = gen_random_n_l_grams(L1, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_4 = recurrent_generation_n_l_grams(L1, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l2_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                // distorted_n_grams_l2_2 = bigram_affine_distortion(&n_gram_l2, &UKR_ALPHABET);
                distorted_n_grams_l2_2 = gen_affine_distortion(&n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_3 = gen_random_n_l_grams(L2, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_4 = recurrent_generation_n_l_grams(L2, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l3_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                // distorted_n_grams_l3_2 = bigram_affine_distortion(&n_gram_l3, &UKR_ALPHABET);
                distorted_n_grams_l3_2 = gen_affine_distortion(&n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_3 = gen_random_n_l_grams(L3, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_4 = recurrent_generation_n_l_grams(L3, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l4_1_r1 = vigenere_cipher_distortion(R1, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_1_r2 = vigenere_cipher_distortion(R2, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_1_r3 = vigenere_cipher_distortion(R3, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                // distorted_n_grams_l4_2 = bigram_affine_distortion(&n_gram_l4, &UKR_ALPHABET);
                distorted_n_grams_l4_2 = gen_affine_distortion(&n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_3 = gen_random_n_l_grams(L4, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_4 = recurrent_generation_n_l_grams(L4, N1, &UKR_ALPHABET, l_little);
            });
        });
        println!("Distorted N grams are made (struct_lzma)");

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = struct_lzma(L1, &n_gram_l1, threshold);
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r1 = struct_lzma(L1, &distorted_n_grams_l1_1_r1.0, threshold);
                println!(
                    "res1_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r2 = struct_lzma(L1, &distorted_n_grams_l1_1_r2.0, threshold);
                println!(
                    "res1_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r3 = struct_lzma(L1, &distorted_n_grams_l1_1_r3.0, threshold);
                println!(
                    "res1_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_2 = struct_lzma(L1, &distorted_n_grams_l1_2.0, threshold);
                println!(
                    "res1_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_3 = struct_lzma(L1, &distorted_n_grams_l1_3, threshold);
                println!(
                    "res1_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_4 = struct_lzma(L1, &distorted_n_grams_l1_4, threshold);
                println!(
                    "res1_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_0 = struct_lzma(L2, &n_gram_l2, threshold);
                println!(
                    "res2_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r1 = struct_lzma(L2, &distorted_n_grams_l2_1_r1.0, threshold);
                println!(
                    "res2_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r2 = struct_lzma(L2, &distorted_n_grams_l2_1_r2.0, threshold);
                println!(
                    "res2_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r3 = struct_lzma(L2, &distorted_n_grams_l2_1_r3.0, threshold);
                println!(
                    "res2_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_2 = struct_lzma(L2, &distorted_n_grams_l2_2.0, threshold);
                println!(
                    "res2_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_3 = struct_lzma(L2, &distorted_n_grams_l2_3, threshold);
                println!(
                    "res2_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_4 = struct_lzma(L2, &distorted_n_grams_l2_4, threshold);
                println!(
                    "res2_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = struct_lzma(L3, &n_gram_l3, threshold);
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = struct_lzma(L3, &distorted_n_grams_l3_1_r1.0, threshold);
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = struct_lzma(L3, &distorted_n_grams_l3_1_r2.0, threshold);
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = struct_lzma(L3, &distorted_n_grams_l3_1_r3.0, threshold);
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = struct_lzma(L3, &distorted_n_grams_l3_2.0, threshold);
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = struct_lzma(L3, &distorted_n_grams_l3_3, threshold);
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = struct_lzma(L3, &distorted_n_grams_l3_4, threshold);
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = struct_lzma(L4, &n_gram_l4, threshold);
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = struct_lzma(L4, &distorted_n_grams_l4_1_r1.0, threshold);
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = struct_lzma(L4, &distorted_n_grams_l4_1_r2.0, threshold);
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = struct_lzma(L4, &distorted_n_grams_l4_1_r3.0, threshold);
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = struct_lzma(L4, &distorted_n_grams_l4_2.0, threshold);
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = struct_lzma(L4, &distorted_n_grams_l4_3, threshold);
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = struct_lzma(L4, &distorted_n_grams_l4_4, threshold);
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

        \n\t (struct_lzma) [l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (struct_lzma) [l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} , \
        \n\t (struct_lzma) [l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (struct_lzma) [l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (struct_lzma) [l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (struct_lzma) [l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (struct_lzma) [l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (struct_lzma) [l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

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
}

fn struct_lzma(
    l: usize,
    l_grams: &Vec<String>,
    threshold: f64,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    for l_gram in l_grams {
        let (mut compressed_normal_text, mut compressed_random_text): (Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());
        rayon::scope(|s| {
            s.spawn(|_s| {
                compressed_normal_text = compress(l_gram.as_bytes());
            });
            s.spawn(|_s| {
                compressed_random_text = compress(gen_random_l_gram_char_alphabet(l, UKR_ALPHABET.len(), &UKR_ALPHABET).as_bytes());
            });
        });
        let compression_coef_normal = l as f64 / compressed_normal_text.len() as f64;
        let compression_coef_random = l as f64 / compressed_random_text.len() as f64;

        if (compression_coef_normal - compression_coef_random).abs() <= threshold {
            h_1 += 1;
        } else {
            h_0 += 1;
        }
    }
    (h_0, h_1)
}

fn compress(data: &[u8]) -> Vec<u8> {
    lzma::compress(data, 6).expect("Failed to finish compression!")
}

#[test]
fn sorting() {
    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let (chunks, threshold) = (100, 10);
    let mut compressed_random_text = Default::default();
    let random_text = gen_random_l_gram_char_alphabet(chunks, UKR_ALPHABET.len(), &UKR_ALPHABET);
    compressed_random_text = compress(random_text.as_bytes());

    println!("{random_text} -- {} -- {:?}", compressed_random_text.len(), compressed_random_text)
}