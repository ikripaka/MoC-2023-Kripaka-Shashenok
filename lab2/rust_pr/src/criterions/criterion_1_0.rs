use crate::internals::{calculate_probs, divide_into_l_grams, double_content, generate_affine_distortion, bigram_affine_distortion, generate_random_n_l_grams, is_n_gram_prohibited_with_custom_l_grams, is_n_gram_prohibited_with_ngrams, make_frequency_table, make_frequency_table_from_file, make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, vigenere_cipher_distortion, make_frequency_table_for_long_chunks, make_frequency_table_custom_manual};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};
use chrono::Local;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let content_for_analysis = double_content(&content);
    // bigram: 992, three:16347, l1:491437
    // bigram: 1000, three:17911, l1:956458
    // bigram: 992, three:16347, l2:52264
    // bigram: 992, three:16347, l2:104529
    // bigram: 992, three:16347, l3:5226


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
    // println!("bigram: {}, three:{}, l3:{}", freq_table_bigram.keys().len(), freq_table_three_gram.keys().len(), freq_table_l3.keys().len());
    println!("Frequency tables are calculated (criterion_1)");

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
    println!("Gained real frequency tables for detection of prohibited n grams (criterion_1)");

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
    println!("Distorted N grams are made (criterion_1)");

    rayon::scope(|s| {
        s.spawn(|_s| {
            res1_0 = criterion_1(
                &freq_table_prh_l1,
                &n_gram_l1,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_1_r1 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_1_r1.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_1_r2 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_1_r2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_1_r3 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_1_r3.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_2 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_3 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_3,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });
        s.spawn(|_s| {
            res1_4 = criterion_1(
                &freq_table_prh_l1,
                &distorted_n_grams_l1_4,
                &vec![L_BIGRAM, L_THREE_GRAM, L1],
                0,
            );
        });

        s.spawn(|_s| {
            res2_0 = criterion_1(
                &freq_table_prh_l2,
                &n_gram_l2,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_1_r1 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_1_r1.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_1_r2 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_1_r2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_1_r3 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_1_r3.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_2 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_3 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_3,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });
        s.spawn(|_s| {
            res2_4 = criterion_1(
                &freq_table_prh_l2,
                &distorted_n_grams_l2_4,
                &vec![L_BIGRAM, L_THREE_GRAM, L2],
                0,
            );
        });

        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_0 = criterion_1(
                &freq_table_prh_l3,
                &n_gram_l3,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_0 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r1 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_1_r1.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r2 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_1_r2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_1_r3 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_1_r3.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_2 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_3 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_3,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                10_000,
            );
            println!(
                "res3_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res3_4 = criterion_1(
                &freq_table_prh_l3,
                &distorted_n_grams_l3_4,
                &vec![L_BIGRAM, L_THREE_GRAM, L3],
                0,
            );
            println!(
                "res3_4 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });

        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_0 = criterion_1(
                &freq_table_prh_l4,
                &n_gram_l4,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_0 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r1 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_1_r1.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_1_r1 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r2 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_1_r2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_1_r2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_1_r3 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_1_r3.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_1_r3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_2 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_2.0,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_2 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_3 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_3,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
            );
            println!(
                "res4_3 FINISHED!! Time:{}",
                (Local::now() - time_prev_local).num_minutes()
            );
        });
        s.spawn(|_s| {
            let time_prev_local = Local::now();
            res4_4 = criterion_1(
                &freq_table_prh_l4,
                &distorted_n_grams_l4_4,
                &vec![L_BIGRAM, L_THREE_GRAM, L4],
                0,
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

        // \n\t (criterion_1) [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        // \n\t (criterion_1) [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        // \n\t (criterion_1) [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1) [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1) [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1) [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
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

fn criterion_1(
    freq_table_prh: &Vec<&HashMap<String, u64>>,
    l_grams: &Vec<String>,
    l_grams_len: &Vec<usize>,
    threshold: u64,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    for l_gram in l_grams {
        let (mut has_prohibited_bigram, mut has_prohibited_three_gram, mut has_prohibited_l_gram) =
            (false, false, false);
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
        if has_prohibited_bigram || has_prohibited_three_gram || has_prohibited_l_gram {
            // println!("1: {has_prohibited_bigram}, 2: {has_prohibited_three_gram}, 3: {has_prohibited_l_gram}");
            h_1 += 1;
        } else {
            h_0 += 1;
        }
    }
    (h_0, h_1)
}