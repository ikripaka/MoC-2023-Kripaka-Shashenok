use crate::internals::{
    calculate_probs, divide_into_l_grams, double_content, gen_affine_distortion,
    gen_random_n_l_grams, make_frequency_table, make_frequency_table_custom_manual,
    make_frequency_table_for_long_chunks, make_frequency_table_from_file, make_n_gram_on_alphabet,
    make_n_gram_on_content_from_str, recurrent_generation_n_l_grams, sort_hash_map_asc,
    vigenere_cipher_distortion,
};
use crate::{L1, L2, L3, L4, L_BIGRAM, L_MONOGRAM, L_THREE_GRAM, N1, N2, R1, R2, R3, UKR_ALPHABET};
use chrono::Local;
use dotenv::dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run(filepath: &str) {
    let time_prev = Local::now();

    let default_threshold = 25;
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
    let freq_tables = vec![&freq_table_monogram, &freq_table_bigram];

    let sorted_map_monogram = sort_hash_map_asc(&freq_table_monogram);
    let sorted_map_bigram = sort_hash_map_asc(&freq_table_bigram);
    let sorted_maps = vec![&sorted_map_monogram, &sorted_map_bigram];

    println!("Frequency tables are calculated (criterion_1_2)");

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
    println!("N grams are made (criterion_1_2)");

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
        let (mut prh_freq_table1, mut prh_freq_table2, mut prh_freq_table3, mut prh_freq_table4) =
            (0, 0, 0, 0);
        let (
            mut freq_table_threshold1,
            mut freq_table_threshold2,
            mut freq_table_threshold3,
            mut freq_table_threshold4,
        ) = (0, 0, 0, 0);
        if l_little == 1 {
            (
                prh_freq_table1,
                prh_freq_table2,
                prh_freq_table3,
                prh_freq_table4,
            ) = (10, 10, 10, 10);
            (
                freq_table_threshold1,
                freq_table_threshold2,
                freq_table_threshold3,
                freq_table_threshold4,
            ) = (0,0,0,0)
        } else if l_little == 2 {
            (
                prh_freq_table1,
                prh_freq_table2,
                prh_freq_table3,
                prh_freq_table4,
            ) = (200, 200, 150, 100);
            (
                freq_table_threshold1,
                freq_table_threshold2,
                freq_table_threshold3,
                freq_table_threshold4,
            ) = (0,0,0,0)
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
        println!("Distorted N grams are made (criterion_1_2)");

        rayon::scope(|s| {
            s.spawn(|_s| {
                res1_0 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &n_gram_l1,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r1 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_1_r1.0,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_1_r2.0,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_1_r3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_1_r3.0,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_2.0,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_3,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res1_4 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l1_4,
                    freq_table_threshold1,
                    prh_freq_table1,
                    l_little as usize,
                );
                println!(
                    "res1_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_0 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &n_gram_l2,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r1 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_1_r1.0,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_1_r2.0,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_1_r3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_1_r3.0,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_2.0,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_3,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                )
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res2_4 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l2_4,
                    freq_table_threshold2,
                    prh_freq_table2,
                    l_little as usize,
                );
                println!(
                    "res2_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_0 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &n_gram_l3,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r1 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_1_r1.0,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_1_r2.0,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_1_r3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_1_r3.0,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_2.0,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_3,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res3_4 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l3_4,
                    freq_table_threshold3,
                    prh_freq_table3,
                    l_little as usize,
                );
                println!(
                    "res3_4 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });

            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_0 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &n_gram_l4,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_0 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r1 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_1_r1.0,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r1 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_1_r2.0,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_1_r3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_1_r3.0,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_1_r3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_2 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_2.0,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_2 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_3 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_3,
                    freq_table_threshold4,
                    prh_freq_table4,
                    l_little as usize,
                );
                println!(
                    "res4_3 FINISHED!! Time:{}",
                    (Local::now() - time_prev_local).num_minutes()
                );
            });
            s.spawn(|_s| {
                let time_prev_local = Local::now();
                res4_4 = criterion_1_2(
                    &sorted_maps,
                    &freq_tables,
                    &distorted_n_grams_l4_4,
                    freq_table_threshold4,
                    prh_freq_table4,
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

        \n\t (criterion_1_2)[l: {l_little}] [res1_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_1_2)[l: {l_little}] [res_1_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_1_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_1_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_1_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} , \
        \n\t (criterion_1_2)[l: {l_little}] [res_1_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \
        \n\t (criterion_1_2)[l: {l_little}] [res_1_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?} \

        \n\t (criterion_1_2)[l: {l_little}] [res_2_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_2_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1_2)[l: {l_little}] [res_3_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_3_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\

        \n\t (criterion_1_2)[l: {l_little}] [res_4_0](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_r1](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_r2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_r3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_2](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_3](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
        \n\t (criterion_1_2)[l: {l_little}] [res_4_4](h0, h1): {:?}, ((p_h_0, p_h_1), (alpha, beta)): {:?}\
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

fn criterion_1_2(
    sorted_grams: &Vec<&Vec<(String, u64)>>,
    freq_tables: &Vec<&HashMap<String, u64>>,
    l_grams: &Vec<String>,
    freq_table_threshold: u64,
    prh_grams_len: usize,
    l_little: usize,
) -> (u64, u64) {
    let (mut h_0, mut h_1) = (0, 0);

    if l_little == L_MONOGRAM {
        let mut prh_monograms = sorted_grams[0].clone();
        prh_monograms.truncate(prh_grams_len);
        for l_gram in l_grams {
            let has_prohibited_monogram = {
                let mut res = false;
                let hash_map = get_prh_l_grams(l_gram, &prh_monograms);
                for (key, val) in hash_map.iter() {
                    // println!("l:{l_little} ({l_gram}) (key: {key}), {val} > {:?}", freq_tables[0].get(key));
                    if *val
                        > match freq_tables[0].get(key) {
                            None => 0,
                            Some(val) => *val,
                        }
                    {
                        res = true;
                        break;
                    }
                }
                res
            };

            // println!("{has_prohibited_bigram}, {has_prohibited_three_gram}, {has_prohibited_l_gram }");
            if has_prohibited_monogram {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    } else if l_little == L_BIGRAM {
        let mut prh_bigrams =
            calc_prh_grams(freq_tables[1], &sorted_grams[1], 2, prh_grams_len);
        prh_bigrams.truncate(prh_grams_len);
        for l_gram in l_grams {
            // лдордивакуватийдиректоргогвортськоїшколичарівтачаклунстваніколинебоявсяпризначатинавчительськіпосадинепевнихосібповідомляєрітаскітернашспеціальнийкореспондентувересніцьогорокувіннайняваласторамудінапрізвиськодикозорсумнозвісноготаневдатногоколишньогоавроращобнавчатидітейзахистувідтемнихмистецтвцерішеннявикликалоподивінерозуміннявміністерствімагіїдоцьогоспричиниласядобревідомазвичкамудінападатинавсякогохтовйогоприсутностізробитьнеобережнийрухдикозормудіодначездаєтьсявідповідальнимідоброзичливимпорядзнапівлюдиноюякудамблдорузяввикладатипредметщоназиваєтьсядоглядзамагічнимиістотамирубеусгегрідзізнаєтьсящовтретьомукласібуввідрахованийзгогвортсувідтодівінвтішавсяпосадоюшкільноголісникаякузанимзакріпивдамблдороднакторікгегрідвикориставсвійтаємничийвпливнадиректоращобнадодачупосістищеймісцевчителядоглядузамагічнимиістотамивідсунувшибагатьохкваліфікованішихвикладачівтривогувикликаютьужесамнеймовірнийзрісттастрахітливийвиглядцьогочоловікагегрідвикористовуєздобутувладущобуспішнозалякуватиучнівза

            let has_prohibited_bigrams = {
                let mut res = false;
                let hash_map = get_prh_l_grams(l_gram, &prh_bigrams);
                for (key, val) in hash_map.iter() {
                    // println!("l:{l_little} ({l_gram}) (key: {key}), {val} > {:?}", freq_tables[1].get(key));
                    if *val
                        > match freq_tables[1].get(key) {
                            None => 0,
                            Some(val) => *val,
                        }
                    {
                        res = true;
                        break;
                    }
                }
                res
            };

            // println!("{has_prohibited_bigram}, {has_prohibited_three_gram}, {has_prohibited_l_gram }");
            if has_prohibited_bigrams {
                h_1 += 1;
            } else {
                h_0 += 1;
            }
        }
    }

    (h_0, h_1)
}

fn calc_prh_grams(
    freq_table: &HashMap<String, u64>,
    sorted_grams: &Vec<(String, u64)>,
    chunks: usize,
    threshold: usize,
) -> Vec<(String, u64)> {
    let alphabet = make_n_gram_on_alphabet(chunks, &UKR_ALPHABET);
    let mut absent_prh_grams = Vec::new();
    for alphabet_gram in alphabet {
        if !sorted_grams.contains(&(
            alphabet_gram.clone(),
            match freq_table.get(&alphabet_gram) {
                None => 0,
                Some(val) => *val,
            },
        )) {
            // println!("don't contain {alphabet_gram}");
            absent_prh_grams.push((alphabet_gram.clone(), 0))
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

fn get_prh_l_grams(n_gram: &str, prh_grams: &Vec<(String, u64)>) -> HashMap<String, u64> {
    let mut prohibited_freq_table = HashMap::new();
    for prh in prh_grams.iter() {
        if n_gram.contains(&prh.0) {
            match prohibited_freq_table.get(&prh.0) {
                None => {
                    prohibited_freq_table.insert(prh.0.clone(), n_gram.matches(&prh.0).count()  as u64)
                },
                Some(val) => prohibited_freq_table.insert(prh.0.clone(), *val + 1),
            };
        };
    }
    prohibited_freq_table
}

#[test]
fn calc_prh_grams_test() {
    let (mut freq_table_monogram, mut freq_table_bigram) = Default::default();
    let not_forbidden = "типопродалисьсподіваючисьзатенагородипанипольськіневипускалисвоїхдочокздо";
    let forbidden = "бтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрпвфцкншщзхїґґбтмчждлрп";

    dotenv().ok();
    let filepath = std::env::var("OUTPUT_FILENAME")
        .unwrap()
        .as_str()
        .to_string();
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
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

    let (threshold1, threshold2) = (10, 200);

    let prh_monograms = calc_prh_grams(&freq_table_monogram, &sorted_map_monogram, 1, threshold1);
    let prh_bigrams = calc_prh_grams(&freq_table_bigram, &sorted_map_bigram, 2, threshold2);
    println!(
        "mono: {:?}, threshold:{} \n\t bi: {:?}, threshold:{}",
        prh_monograms, threshold1, prh_bigrams, threshold2
    );
    println!("bi len: {:?}", prh_bigrams.len());

    println!(
        "is_forbidden forbidden: {:?} \n\t\t is_forbidden real: {:?}",
        // is_n_gram_forbidden_with_custom_l_grams(forbidden, &freq_table,threshold, chunks),
        get_prh_l_grams(forbidden, &prh_bigrams),
        // "",
        get_prh_l_grams(not_forbidden, &prh_bigrams),
    );
}
