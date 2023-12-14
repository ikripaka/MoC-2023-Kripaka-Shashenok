use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use chrono::Local;
use dotenv::dotenv;

use crate::internals::{calculate_entropy, calculate_probs, divide_into_l_grams, double_content, gen_affine_distortion, gen_random_n_l_grams, make_frequency_table, make_frequency_table_custom_manual, make_frequency_table_for_long_chunks, make_frequency_table_from_file, make_probability_table, recurrent_generation_n_l_grams, vigenere_cipher_distortion};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_MONOGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};

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
    println!("Frequency tables are calculated (criterion_3_0)");

    let (
        mut probs_monogram,
        mut probs_bigram,
    ) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            probs_monogram =
                make_probability_table(&freq_table_monogram);
            println!("probs_monogram DONE")
        });
        s.spawn(|_s| {
            probs_bigram =
                make_probability_table(&freq_table_bigram);
            println!("probs_bigram DONE")
        });
    });

    let (
        mut h_l_1,
        mut h_l_2,
    ) = Default::default();
    rayon::scope(|s| {
        s.spawn(|_s| {
            h_l_1 =
                calc_h_l(L_MONOGRAM, &probs_monogram);
            println!("h_l_1 DONE")
        });
        s.spawn(|_s| {
            h_l_2 =
                calc_h_l(L_BIGRAM, &probs_bigram);
            println!("h_l_2 DONE")
        });
    });
    let h_l = vec![h_l_1, h_l_2];


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
    println!("N grams are made (criterion_3_0)");

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


    for l_little in 2..=2 {
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
        println!("Distorted N grams are made (criterion_3_0)");

        let (mut k_h_1 ,mut k_h_2, mut k_h_3, mut k_h_4) = Default::default();
        if l_little == 1{
            (k_h_1 , k_h_2,  k_h_3,  k_h_4) = (1.7, 0.3, 0.25, 0.1);
        } else if l_little == 2{
            (k_h_1 , k_h_2,  k_h_3,  k_h_4) = (3.23, 1.45, 0.69, 0.13);
        }

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = criterion_3_0(&n_gram_l1, &h_l, k_h_1, l_little);
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r1 = criterion_3_0(
                    &distorted_n_grams_l1_1_r1.0,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r2 = criterion_3_0(
                    &distorted_n_grams_l1_1_r2.0,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r3 = criterion_3_0(
                    &distorted_n_grams_l1_1_r3.0,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_2 = criterion_3_0(
                    &distorted_n_grams_l1_2.0,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_3 = criterion_3_0(
                    &distorted_n_grams_l1_3,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_4 = criterion_3_0(
                    &distorted_n_grams_l1_4,
                    &h_l, k_h_1, l_little,
                );
                println!(
                    "res1_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_0 = criterion_3_0(
                    &n_gram_l2,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r1 = criterion_3_0(
                    &distorted_n_grams_l2_1_r1.0,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r2 = criterion_3_0(
                    &distorted_n_grams_l2_1_r2.0,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r3 = criterion_3_0(
                    &distorted_n_grams_l2_1_r3.0,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_2 = criterion_3_0(
                    &distorted_n_grams_l2_2.0,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_3 = criterion_3_0(
                    &distorted_n_grams_l2_3,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_4 = criterion_3_0(
                    &distorted_n_grams_l2_4,
                    &h_l, k_h_2, l_little,
                );
                println!(
                    "res2_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = criterion_3_0(
                    &n_gram_l3,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = criterion_3_0(
                    &distorted_n_grams_l3_1_r1.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = criterion_3_0(
                    &distorted_n_grams_l3_1_r2.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = criterion_3_0(
                    &distorted_n_grams_l3_1_r3.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = criterion_3_0(
                    &distorted_n_grams_l3_2.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = criterion_3_0(
                    &distorted_n_grams_l3_3,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = criterion_3_0(
                    &distorted_n_grams_l3_4,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = criterion_3_0(
                    &n_gram_l4,
                    &h_l, k_h_4, l_little,
                );
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = criterion_3_0(
                    &distorted_n_grams_l4_1_r1.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = criterion_3_0(
                    &distorted_n_grams_l4_1_r2.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = criterion_3_0(
                    &distorted_n_grams_l4_1_r3.0,
                    &h_l, k_h_3, l_little
                    ,
                );
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = criterion_3_0(
                    &distorted_n_grams_l4_2.0,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = criterion_3_0(
                    &distorted_n_grams_l4_3,
                    &h_l, k_h_3, l_little,
                );
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = criterion_3_0(
                    &distorted_n_grams_l4_4,
                    &h_l, k_h_3, l_little,
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

        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} , \
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_3_0) [k_p: {k_h_1}] [l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_2}] [l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_3}] [l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_3_0) [k_p: {k_h_4}] [l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
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

fn criterion_3_0(
    l_grams: &Vec<String>,
    h_l: &Vec<f64>,
    k_h: f64,
    l_little: u8,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    if l_little == L_MONOGRAM as u8 {
        for l_gram in l_grams {
            let probs_local = make_probability_table(&make_frequency_table(l_gram, L_MONOGRAM));
            let h_l_local = calc_h_l(L_MONOGRAM, &probs_local);
            // println!("0: {} -- {h_l_local}", h_l[0]);
            if (h_l[0] - h_l_local).abs() > k_h {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    } else if l_little == L_BIGRAM as u8 {
        for l_gram in l_grams {
            let probs_local = make_probability_table(&make_frequency_table_for_long_chunks(l_gram, L_BIGRAM, 0..L_BIGRAM));
            let h_l_local = calc_h_l(L_BIGRAM, &probs_local);
            // println!("1: {} -- {h_l_local}", h_l[0]);
            if (h_l[1] - h_l_local).abs() > k_h {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    }


    (h_0, h_1)
}

fn calc_h_l(l_little: usize, probs: &HashMap<String, f64>) -> f64 {
    calculate_entropy(probs) / l_little as f64
}

// #[test]
// fn custom_forbidden_grams_test() {
//     let not_forbidden = "типопродалисьсподіваючисьзатенагородипанипольськіневипускалисвоїхдочокздо";
//     let forbidden = "бтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрп";
//
//     dotenv().ok();
//     let filepath = std::env::var("OUTPUT_FILENAME")
//         .unwrap()
//         .as_str()
//         .to_string();
//     let (chunks, threshold) = (2, 10);
//     let freq_table = make_frequency_table_from_file(&filepath, chunks);
//     let time_prev = Local::now();
//     println!(
//         "is_forbidden forbidden: {:?} \n\t\t is_forbidden real: {:?}",
//         // is_n_gram_forbidden_with_custom_l_grams(forbidden, &freq_table,threshold, chunks),
//         is_n_gram_prohibited_with_custom_l_grams(forbidden, &freq_table, threshold, chunks),
//         // "",
//         is_n_gram_prohibited_with_custom_l_grams(not_forbidden, &freq_table, threshold, chunks)
//     );
//     let time_after = Local::now();
//     println!("{}", (time_after - time_prev).num_milliseconds())
// }
