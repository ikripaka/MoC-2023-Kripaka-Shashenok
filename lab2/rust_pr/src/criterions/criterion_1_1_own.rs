use crate::internals::{gen_affine_distortion, calculate_probs, divide_into_l_grams, double_content, gen_random_n_l_grams, make_frequency_table, make_frequency_table_custom_manual, make_frequency_table_for_long_chunks, make_frequency_table_from_file, make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, vigenere_cipher_distortion};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};
use chrono::Local;
use dotenv::dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let default_threshold = 0;
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let content_for_analysis = double_content(&content);

    // для l1 заборонених біграм до 20
    // для l2 заборонених біграм 38, до 56
    // для l3 заборонених біграм 490
    // для l4 заборонених біграм 5266

    // #0
    let (
        mut res1_0,
        mut res1_1_r1,
        mut res1_1_r2,
        mut res1_1_r3,
        mut res1_2,
        mut res1_3,
        mut res1_4,
    ): ((u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64)) = Default::default();
    let (
        mut res2_0,
        mut res2_1_r1,
        mut res2_1_r2,
        mut res2_1_r3,
        mut res2_2,
        mut res2_3,
        mut res2_4,
    ): ((u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64)) = Default::default();

    let (
        mut res3_0,
        mut res3_1_r1,
        mut res3_1_r2,
        mut res3_1_r3,
        mut res3_2,
        mut res3_3,
        mut res3_4,
    ): ((u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64)) = Default::default();
    let (
        mut res4_0,
        mut res4_1_r1,
        mut res4_1_r2,
        mut res4_1_r3,
        mut res4_2,
        mut res4_3,
        mut res4_4,
    ): ((u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64), (u64, u64)) = Default::default();

    // calculating frequency tables for text
    let (
        mut freq_table_bigram,
        mut freq_table_three_gram,
        mut freq_table_l1,
        mut freq_table_l2,
        mut freq_table_l3,
        mut freq_table_l4,
    ) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            freq_table_bigram = make_frequency_table_for_long_chunks(&content, L_BIGRAM, 0..L_BIGRAM);
            println!("freq_table_bigram DONE")
        });
        s.spawn(|_s| {
            freq_table_three_gram = make_frequency_table_for_long_chunks(&content, L_THREE_GRAM, 0..L_THREE_GRAM);
            println!("freq_table_three_gram DONE")
        });
        s.spawn(|_s| {
            freq_table_l1 = make_frequency_table_for_long_chunks(&content, L1, 0..L1);
            println!("freq_table_l1 DONE")
        });
        s.spawn(|_s| {
            freq_table_l2 = make_frequency_table_for_long_chunks(&content, L2, 0..L2);
            println!("freq_table_l2 DONE")
        });
        s.spawn(|_s| {
            freq_table_l3 = make_frequency_table_custom_manual(&content, L3);
            println!("freq_table_l3 DONE")
        });
        s.spawn(|_s| {
            freq_table_l4 = make_frequency_table_custom_manual(&content, L4);
            println!("freq_table_l4 DONE")
        });
    });
    println!("Frequency tables are calculated (criterion_1_1)");

    // gaining real frequency tables for detection of prohibited n grams
    let (
        mut freq_table_prh_l1,
        mut freq_table_prh_l2,
        mut freq_table_prh_l3,
        mut freq_table_prh_l4,
    ) = Default::default();
    freq_table_prh_l1 = vec![&freq_table_bigram, &freq_table_three_gram, &freq_table_l1];
    freq_table_prh_l2 = vec![&freq_table_bigram, &freq_table_three_gram, &freq_table_l2];
    freq_table_prh_l3 = vec![&freq_table_bigram, &freq_table_three_gram, &freq_table_l3];
    freq_table_prh_l4 = vec![&freq_table_bigram, &freq_table_three_gram, &freq_table_l4];
    println!("Gained real frequency tables for detection of prohibited n grams (criterion_1_1)");

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
    println!("N grams are made (criterion_1_1)");

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
                distorted_n_grams_l4_2 = gen_affine_distortion(&n_gram_l4, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_3 = gen_random_n_l_grams(L4, N2, &UKR_ALPHABET, l_little);
            });
            s.spawn(|_s| {
                distorted_n_grams_l4_4 = recurrent_generation_n_l_grams(L4, N2, &UKR_ALPHABET, l_little);
            });
        });
        println!("Distorted N grams are made (criterion_1_1)");

        let k_p_1 = 9;
        let k_p_2 = 38;
        let k_p_3 = 496;
        let k_p_4 = 5240;

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = criterion_1_1(
                    &freq_table_prh_l1,
                    &n_gram_l1,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r1 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_1_r1.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r2 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_1_r2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r3 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_1_r3.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_2 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_3 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_3,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_4 = criterion_1_1(
                    &freq_table_prh_l1,
                    &distorted_n_grams_l1_4,
                    &vec![L_BIGRAM, L_THREE_GRAM, L1],
                    default_threshold,
                    k_p_1,
                );
                println!(
                    "res1_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_0 = criterion_1_1(
                    &freq_table_prh_l2,
                    &n_gram_l2,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r1 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_1_r1.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r2 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_1_r2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r3 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_1_r3.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_2 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_3 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_3,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_4 = criterion_1_1(
                    &freq_table_prh_l2,
                    &distorted_n_grams_l2_4,
                    &vec![L_BIGRAM, L_THREE_GRAM, L2],
                    default_threshold,
                    k_p_2,
                );
                println!(
                    "res2_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = criterion_1_1(
                    &freq_table_prh_l3,
                    &n_gram_l3,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_1_r1.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_1_r2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_1_r3.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_3,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = criterion_1_1(
                    &freq_table_prh_l3,
                    &distorted_n_grams_l3_4,
                    &vec![L_BIGRAM, L_THREE_GRAM, L3],
                    default_threshold,
                    k_p_3,
                );
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = criterion_1_1(
                    &freq_table_prh_l4,
                    &n_gram_l4,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_1_r1.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_1_r2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_1_r3.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_2.0,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_3,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
                );
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = criterion_1_1(
                    &freq_table_prh_l4,
                    &distorted_n_grams_l4_4,
                    &vec![L_BIGRAM, L_THREE_GRAM, L4],
                    default_threshold,
                    k_p_4,
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

        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} , \
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_1_1) [k_p: {k_p_1}] [l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_2}] [l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_3}] [l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_1) [k_p: {k_p_4}] [l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
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

fn criterion_1_1(
    freq_table_prh: &Vec<&HashMap<String, u64>>,
    l_grams: &Vec<String>,
    l_grams_len: &Vec<usize>,
    threshold: u64,
    prohibited_grams_threshold: u64,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    for l_gram in l_grams {
        let (mut has_prohibited_bigram, mut has_prohibited_three_gram, mut has_prohibited_l_gram) =
            (0, 0, 0);
        rayon::scope(|s| {
            s.spawn(|_s| {
                has_prohibited_bigram = is_n_gram_prohibited_with_custom_l_grams(
                    l_gram,
                    freq_table_prh[0],
                    threshold,
                    l_grams_len[0],
                );
            });
            s.spawn(|_s| {
                has_prohibited_three_gram = is_n_gram_prohibited_with_custom_l_grams(
                    l_gram,
                    freq_table_prh[1],
                    threshold,
                    l_grams_len[1],
                );
            });
            s.spawn(|_s| {
                has_prohibited_l_gram =
                    is_n_gram_prohibited_with_ngrams(l_gram, freq_table_prh[2], threshold);
            });
        });
        // println!("{} {prohibited_grams_threshold}", has_prohibited_bigram + has_prohibited_three_gram + has_prohibited_l_gram);
        if has_prohibited_bigram + has_prohibited_three_gram + has_prohibited_l_gram
            >= prohibited_grams_threshold
        {
            h_1 += 1;
        } else {
            h_0 += 1;
        }
    }
    (h_0, h_1)
}

fn is_n_gram_prohibited_with_ngrams(
    n_gram: &str,
    frequency_table: &HashMap<String, u64>,
    threshold: u64,
) -> u64 {
    match frequency_table.get(n_gram) {
        None => 1,
        Some(val) => {
            if *val < threshold {
                return 1;
            }
            0
        }
    }
}

fn is_n_gram_prohibited_with_custom_l_grams(
    n_gram: &str,
    frequency_table: &HashMap<String, u64>,
    threshold: u64,
    chunks: usize,
) -> u64 {
    let mut prohibited_grams = 0;
    for i in 0..chunks {
        let mut counter = n_gram.chars().into_iter().count() - i;
        let mut char_iter = n_gram.chars().into_iter();
        let _skip = (0..i)
            .into_iter()
            .map(|_| char_iter.next().unwrap())
            .collect::<String>();
        while counter > chunks {
            match frequency_table.get(
                &(0..chunks)
                    .into_iter()
                    .map(|x| match char_iter.next() {
                        None => 'a',
                        Some(c) => c,
                    })
                    .collect::<String>(),
            ) {
                None => prohibited_grams += 1,
                Some(val) => {
                    if *val < threshold {
                        prohibited_grams += 1;
                    }
                    // false
                    // then looking for another chunk_grams
                }
            }
            counter -= chunks;
        }
    }
    prohibited_grams
}

#[test]
fn custom_forbidden_grams_test() {
    let not_forbidden = "типопродалисьсподіваючисьзатенагородипанипольськіневипускалисвоїхдочокздо";
    let forbidden = "бтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрп";

    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let (chunks, threshold) = (2, 10);
    let freq_table = make_frequency_table_from_file(&filepath, chunks);
    let time_prev = Local::now();
    println!(
        "is_forbidden forbidden: {:?} \n\t\t is_forbidden real: {:?}",
        // is_n_gram_forbidden_with_custom_l_grams(forbidden, &freq_table,threshold, chunks),
        is_n_gram_prohibited_with_custom_l_grams(forbidden, &freq_table, threshold, chunks),
        // "",
        is_n_gram_prohibited_with_custom_l_grams(not_forbidden, &freq_table, threshold, chunks)
    );
    let time_after = Local::now();
    println!("{}", (time_after - time_prev).num_milliseconds())
}
