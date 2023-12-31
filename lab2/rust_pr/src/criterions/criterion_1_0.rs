use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use chrono::Local;
use dotenv::dotenv;

use crate::internals::{calculate_probs, divide_into_l_grams, double_content, gen_affine_distortion, gen_random_n_l_grams, is_n_gram_prohibited_with_custom_l_grams, is_n_gram_prohibited_with_ngrams, make_frequency_table_custom_manual, make_frequency_table_for_long_chunks, make_frequency_table_from_file, make_n_gram_on_alphabet, make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, sort_hash_map_asc, vigenere_cipher_distortion};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_MONOGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let content_for_analysis = double_content(&content);

    // #0
    let (
        mut res1_0,
        mut res1_1_r1,
        mut res1_1_r2,
        mut res1_1_r3,
        mut res1_2,
        mut res1_3,
        mut res1_4,
    ): (
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
    ) = Default::default();
    let (
        mut res2_0,
        mut res2_1_r1,
        mut res2_1_r2,
        mut res2_1_r3,
        mut res2_2,
        mut res2_3,
        mut res2_4,
    ): (
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
    ) = Default::default();

    let (
        mut res3_0,
        mut res3_1_r1,
        mut res3_1_r2,
        mut res3_1_r3,
        mut res3_2,
        mut res3_3,
        mut res3_4,
    ): (
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
    ) = Default::default();
    let (
        mut res4_0,
        mut res4_1_r1,
        mut res4_1_r2,
        mut res4_1_r3,
        mut res4_2,
        mut res4_3,
        mut res4_4,
    ): (
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
    ) = Default::default();

    // calculating frequency tables for text
    let (
        mut freq_table_monogram,
        mut freq_table_bigram,
    ) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            freq_table_monogram =
                make_frequency_table_for_long_chunks(&content, L_MONOGRAM, 0..L_MONOGRAM);
            println!("freq_table_three_gram DONE")
        });
        s.spawn(|_s| {
            freq_table_bigram =
                make_frequency_table_for_long_chunks(&content, L_BIGRAM, 0..L_BIGRAM);
            println!("freq_table_bigram DONE")
        });
    });

    let sorted_map_monogram = sort_hash_map_asc(&freq_table_monogram);
    let sorted_map_bigram = sort_hash_map_asc(&freq_table_bigram);
    let sorted_maps = vec![&sorted_map_monogram, &sorted_map_bigram];

    // println!("bigram: {}, three:{}, l3:{}", freq_table_bigram.keys().len(), freq_table_three_gram.keys().len(), freq_table_l3.keys().len());
    println!("Frequency tables are calculated (criterion_1)");

    let (mut n_gram_l1, mut n_gram_l2, mut n_gram_l3, mut n_gram_l4): (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ) = (vec![], vec![], vec![], vec![]);
    divide_into_l_grams(
        &mut n_gram_l1,
        &mut n_gram_l2,
        &mut n_gram_l3,
        &mut n_gram_l4,
        &content_for_analysis,
    );
    println!("N grams are made (criterion_1)");

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

    // Iterating over values 1, 2
    for l_little in 1..=2 {
        let (mut threshold1, mut threshold2, mut threshold3, mut threshold4) = (0,0,0,0);
        if l_little == 1{
            (threshold1, threshold2, threshold3, threshold4) = (8, 3, 3, 3)
        }else if l_little == 2{
            (threshold1, threshold2, threshold3, threshold4) = (135, 100, 100, 80)
        }

        rayon::scope(|s| {
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r1 =
                    vigenere_cipher_distortion(R1, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r2 =
                    vigenere_cipher_distortion(R2, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_1_r3 =
                    vigenere_cipher_distortion(R3, &n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_2 = gen_affine_distortion(&n_gram_l1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_3 = gen_random_n_l_grams(L1, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l1_4 =
                    recurrent_generation_n_l_grams(L1, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l2_1_r1 =
                    vigenere_cipher_distortion(R1, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_1_r2 =
                    vigenere_cipher_distortion(R2, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_1_r3 =
                    vigenere_cipher_distortion(R3, &n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_2 = gen_affine_distortion(&n_gram_l2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_3 = gen_random_n_l_grams(L2, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l2_4 =
                    recurrent_generation_n_l_grams(L2, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l3_1_r1 =
                    vigenere_cipher_distortion(R1, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_1_r2 =
                    vigenere_cipher_distortion(R2, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_1_r3 =
                    vigenere_cipher_distortion(R3, &n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_2 = gen_affine_distortion(&n_gram_l3, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_3 = gen_random_n_l_grams(L3, N1, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l3_4 =
                    recurrent_generation_n_l_grams(L3, N1, &UKR_ALPHABET, l_little);
            });

            s.spawn(|_s| {
                distorted_n_grams_l4_1_r1 =
                    vigenere_cipher_distortion(R1, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_1_r2 =
                    vigenere_cipher_distortion(R2, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_1_r3 =
                    vigenere_cipher_distortion(R3, &n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_2 = gen_affine_distortion(&n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_3 = gen_random_n_l_grams(L4, N2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_4 =
                    recurrent_generation_n_l_grams(L4, N2, &UKR_ALPHABET, l_little);
            });
        });
        println!("Distorted N grams are made (criterion_1)");

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = criterion_1(
                    &sorted_maps,
                    &n_gram_l1,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_1_r1 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r1.0,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_1_r2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r2.0,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_1_r3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r3.0,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_2.0,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_3,

                    threshold1,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res1_4 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l1_4,

                    threshold1,
                    l_little,
                );
            });

            s.spawn(|_s| {
                res2_0 = criterion_1(
                    &sorted_maps,
                    &n_gram_l2,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_1_r1 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r1.0,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_1_r2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r2.0,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_1_r3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r3.0,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_2.0,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_3,

                    threshold2,
                    l_little,
                );
            });
            s.spawn(|_s| {
                res2_4 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l2_4,

                    threshold2,
                    l_little,
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = criterion_1(
                    &sorted_maps,
                    &n_gram_l3,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r1.0,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r2.0,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r3.0,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_2.0,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_3,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l3_4,

                    threshold3,
                    l_little,
                );
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = criterion_1(
                    &sorted_maps,
                    &n_gram_l4,

                    threshold4,
                    l_little,
                );
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r1.0,

                    threshold4,
                    l_little,
                );
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r2.0,

                    threshold4,
                    l_little,
                );
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r3.0,
                    threshold4,
                    l_little,
                );
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_2.0,

                    threshold4,
                    l_little,
                );
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_3,

                    threshold4,
                    l_little,
                );
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = criterion_1(
                    &sorted_maps,
                    &distorted_n_grams_l4_4,

                    threshold4,
                    l_little,
                );
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

        // \n\t (criterion_1) [l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        // \n\t (criterion_1) [l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1) [l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1) [l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
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

fn criterion_1(
    sorted_grams: &Vec<&Vec<(String, u64)>>,
    l_grams: &Vec<String>,
    prh_grams_len: usize,
    l_little: u8,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    if l_little == 1 {
        let mut prh_monograms = sorted_grams[0].clone();
        prh_monograms.truncate(prh_grams_len);
        let prh_monograms: Vec<String> = prh_monograms.iter().map(|(str, _)| str.clone()).collect();

        for l_gram in l_grams {
            let has_prohibited_monogram = is_n_gram_has_prh_gram_custom(
                l_gram,
                &prh_monograms,
                L_MONOGRAM,
            );

            if has_prohibited_monogram {
                // println!("1: {has_prohibited_bigram}, 2: {has_prohibited_three_gram}, 3: {has_prohibited_l_gram}");
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    } else if l_little == 2 {

        let mut prh_bigrams = calc_prh_grams(&sorted_grams[1].iter().map(|(str, _)| str.clone()).collect(), 2, prh_grams_len);
        prh_bigrams.truncate(prh_grams_len);
        for l_gram in l_grams {
            let has_prohibited_bigram = is_n_gram_has_prh_gram_custom(
                l_gram,
                &prh_bigrams,
                L_BIGRAM,
            );

            if has_prohibited_bigram {
                // println!("1: {has_prohibited_bigram}, 2: {has_prohibited_three_gram}, 3: {has_prohibited_l_gram}");
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    }
    (h_0, h_1)
}

fn calc_prh_grams(sorted_grams: &Vec<String>, chunks: usize, threshold: usize) -> Vec<String> {
    let alphabet = make_n_gram_on_alphabet(chunks, &UKR_ALPHABET);
    let mut absent_prh_grams = Vec::new();
    for alphabet_gram in alphabet {
        if !sorted_grams.contains(&alphabet_gram) {
            // println!("don't contain {alphabet_gram}");
            absent_prh_grams.push(alphabet_gram.clone())
        }
    }
    if threshold < absent_prh_grams.len() {
        absent_prh_grams.truncate(threshold);
    } else if threshold > absent_prh_grams.len() {
        for i in 0..threshold - absent_prh_grams.len() {
            absent_prh_grams.push(sorted_grams[i].clone())
        }
    }
    absent_prh_grams
}

fn is_n_gram_has_prh_gram_custom(l_gram: &String, prh_grams: &Vec<String>, chunks_len: usize) -> bool {
    for chunk in make_n_gram_on_content_from_str(chunks_len, l_gram) {
        for s in prh_grams {
            if chunk.contains(s) {
                return true;
            }
        }
    }
    false
}

#[test]
fn calc_prh_grams_test(){
    let (
        mut freq_table_monogram,
        mut freq_table_bigram,
    ) = Default::default();

    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let mut file = File::open(filepath).unwrap();
    let mut content =String::new();
    file.read_to_string(&mut content);

    rayon::scope(|s| {
        s.spawn(|_s| {
            freq_table_monogram =
                make_frequency_table_for_long_chunks(&content, L_MONOGRAM, 0..L_MONOGRAM);
            println!("freq_table_three_gram DONE")
        });
        s.spawn(|_s| {
            freq_table_bigram =
                make_frequency_table_for_long_chunks(&content, L_BIGRAM, 0..L_BIGRAM);
            println!("freq_table_bigram DONE")
        });
    });

    let sorted_map_monogram = sort_hash_map_asc(&freq_table_monogram);
    let sorted_map_bigram = sort_hash_map_asc(&freq_table_bigram);

    let sorted_map_monogram: Vec<String> = sorted_map_monogram.iter().map(|(str, _)| str.clone()).collect();
    let sorted_map_bigram: Vec<String> = sorted_map_bigram.iter().map(|(str, _)| str.clone()).collect();

    let (threshold1, threshold2) = (10, 135);

    println!("mono: {:?}, threshold:{} \n\t bi: {:?}, threshold:{}", calc_prh_grams(&sorted_map_monogram, 1, threshold1), threshold1, calc_prh_grams(&sorted_map_bigram, 2, threshold2), threshold2 );
    println!("bi len: {:?}",  calc_prh_grams(&sorted_map_bigram, 2, threshold2).len());
}
