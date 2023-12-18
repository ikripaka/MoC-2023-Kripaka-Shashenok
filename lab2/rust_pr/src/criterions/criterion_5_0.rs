use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use chrono::Local;
use dotenv::dotenv;

use crate::internals::{calculate_probs, double_content, gen_affine_distortion, gen_random_n_l_grams, make_frequency_table, make_frequency_table_custom_manual, make_frequency_table_for_long_chunks, make_frequency_table_from_file, make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, sort_hash_map_asc, sort_hash_map_desc, vigenere_cipher_distortion};
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
    let (mut freq_table_monogram, mut freq_table_bigram) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            freq_table_monogram =
                make_frequency_table_for_long_chunks(&content, L_MONOGRAM, 0..L_MONOGRAM);
            println!("freq_table_monogram DONE")
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
            bigrams = make_n_gram_on_content_from_str(L_BIGRAM, &content_for_analysis);
        });
        s.spawn(|_s| {
            three_grams = make_n_gram_on_content_from_str(L_THREE_GRAM, &content_for_analysis);
        });
        s.spawn(|_s| {
            n_gram_l1 = make_n_gram_on_content_from_str(L1, &content_for_analysis);
        });
        s.spawn(|_s| {
            n_gram_l2 = make_n_gram_on_content_from_str(L2, &content_for_analysis);
        });
        s.spawn(|_s| {
            n_gram_l3 = make_n_gram_on_content_from_str(L3, &content_for_analysis);
        });
        s.spawn(|_s| {
            n_gram_l4 = make_n_gram_on_content_from_str(L4, &content_for_analysis);
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

    for l_little in 1..=2 {
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
                // n_gram_l1.truncate(N1);
                // distorted_n_grams_l1_2 = bigram_affine_distortion(&n_gram_l1, &UKR_ALPHABET);
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
                // distorted_n_grams_l2_2 = bigram_affine_distortion(&n_gram_l2, &UKR_ALPHABET);
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
                // distorted_n_grams_l3_2 = bigram_affine_distortion(&n_gram_l3, &UKR_ALPHABET);
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
                // distorted_n_grams_l4_2 = bigram_affine_distortion(&n_gram_l4, &UKR_ALPHABET);
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
        println!("Distorted N grams are made (criterion_5)");

        let (mut boxes1, mut boxes2, mut boxes3, mut boxes4) = (0, 0, 0, 0);
        let (mut k_empty_1, mut k_empty_2, mut k_empty_3, mut k_empty_4) = (0, 0, 0, 0);
        if l_little as usize == L_MONOGRAM {
            (boxes1, boxes2, boxes3, boxes4) = (9, 9, 10, 10);
            (k_empty_1, k_empty_2, k_empty_3, k_empty_4) =
                (boxes1 - 1, boxes2 - 2, boxes3 - 7, boxes4 - 8);
        } else if l_little as usize == L_BIGRAM {
            (boxes1, boxes2, boxes3, boxes4) = (50, 100, 100, 200);
            (k_empty_1, k_empty_2, k_empty_3, k_empty_4) =
                (boxes1 - 4, boxes2 - 95, boxes3 - 92, boxes4 - 12);
        }

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = criterion_5(
                    &sorted_maps,
                    &n_gram_l1,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r1 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r1.0,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r2.0,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_1_r3.0,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_2.0,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_3,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_4 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l1_4,
                    boxes1,
                    k_empty_1,
                    l_little as usize,
                );
                println!(
                    "res1_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_0 = criterion_5(
                    &sorted_maps,
                    &n_gram_l2,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r1 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r1.0,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r2.0,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_1_r3.0,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_2.0,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_3,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_4 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l2_4,
                    boxes2,
                    k_empty_2,
                    l_little as usize,
                );
                println!(
                    "res2_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = criterion_5(
                    &sorted_maps,
                    &n_gram_l3,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r1.0,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r2.0,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_1_r3.0,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_2.0,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_3,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l3_4,
                    boxes3,
                    k_empty_3,
                    l_little as usize,
                );
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = criterion_5(
                    &sorted_maps,
                    &n_gram_l4,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r1.0,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r2.0,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_1_r3.0,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_2.0,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_3,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
                );
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = criterion_5(
                    &sorted_maps,
                    &distorted_n_grams_l4_4,
                    boxes4,
                    k_empty_4,
                    l_little as usize,
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

        \n\t (criterion_5) [l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_5) [l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} ,\
        \n\t (criterion_5) [l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_5) [l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (criterion_5) [l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_5) [l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_5) [l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_5) [l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

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

fn criterion_5(
    sorted_grams: &Vec<&Vec<(String, u64)>>,
    l_grams: &Vec<String>,
    boxes: usize,
    k_empty: usize,
    l_little: usize,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    if l_little == L_MONOGRAM {
        let mut monograms_b_prh = sorted_grams[0].clone();
        monograms_b_prh.truncate(boxes);

        for l_gram in l_grams {
            let (mut monogram_crates): Vec<usize> = vec![0; boxes];
            for (i, prh_bigram) in monograms_b_prh.iter().enumerate() {
                if l_gram.contains(&prh_bigram.0) {
                    monogram_crates[i] += 1;
                }
            }

            let f_empty = monogram_crates
                .iter()
                .fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            // println!("({l_little}) {f_empty} >= {k_empty} ");
            if f_empty >= k_empty {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    } else if l_little == L_BIGRAM {
        let mut bigrams_b_prh = sorted_grams[1].clone();
        bigrams_b_prh.truncate(boxes);

        for l_gram in l_grams {
            let (mut bigram_crates): Vec<usize> = vec![0; boxes];
            for (i, prh_bigram) in bigrams_b_prh.iter().enumerate() {
                if l_gram.contains(&prh_bigram.0) {
                    bigram_crates[i] += 1;
                }
            }

            let f_empty = bigram_crates
                .iter()
                .fold(0, |acc, &y| if y == 0 { acc + 1 } else { acc });
            // println!("({l_little}) {f_empty} >= {k_empty} ");
            if f_empty >= k_empty {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    }

    (h_0, h_1)
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
    println!("{:?}", sort_hash_map_asc(&freq_table))
}
