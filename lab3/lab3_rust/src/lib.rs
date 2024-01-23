use chrono::Local;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use std::ops::Index;

const E_65537: u32 = 0x10001;

// x = a (mod n)
#[derive(Clone, Debug)]
pub struct ModuleEquation {
    a: BigUint,
    n: BigUint,
}

// Algorithm to find inverse by module using Extended Euclides algorithm
pub fn inverse(a: &BigInt, n: &BigInt) -> Result<BigUint, &'static str> {
    let mut a_mut = a.clone();
    if a >= n {
        a_mut %= n;
    }

    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a_mut.clone();
    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        let new_t_aux = t;
        t = new_t.clone();
        new_t = new_t_aux - &quotient * &new_t;
        let new_r_aux = r; //auxiliary
        r = new_r.clone();
        new_r = new_r_aux - &quotient * &new_r;
    }
    if r > BigInt::one() {
        return Err("number is not invertible");
    }
    if t < BigInt::zero() {
        t += n;
    }
    Ok(t.to_biguint().unwrap())
}

// solving module equations by using (Generalized Chinese Remainder Theorem)
fn solve_equations(
    equations_vec: &Vec<ModuleEquation>,
    n: &BigUint,
) -> Result<BigUint, &'static str> {
    let mut m_i = vec![BigUint::zero(); equations_vec.len()];
    let mut m = BigUint::one();

    for equation in equations_vec.iter() {
        m *= &equation.n;
    }
    for i in 0..equations_vec.len() {
        m_i[i] = &m / &equations_vec[i].n;
    }

    let mut n_i = Vec::new();
    for i in 0..equations_vec.len() {
        n_i.push(
            inverse(
                &BigInt::from(m_i[i].clone()),
                &BigInt::from(equations_vec[i].n.clone()),
            )
            .unwrap(),
        )
    }

    let mut x = BigUint::zero();
    for i in 0..equations_vec.len() {
        x += (&equations_vec[i].a * &m_i[i] * &n_i[i]) % n;
    }

    return Ok(x % n);
}

pub fn get_string_hex_array(arr: &[u8]) -> String {
    let mut res = format!("{:02X?}", arr);
    res = res.replace(", ", "");
    res = res.trim_start_matches("[").to_string();
    res = res.trim_end_matches("]").to_string();
    res
}

//     let s1 = y.modpow(&((p + one::<BigUint>()) / &four), p);

pub fn perform_hastad_broadcast_attack(c_i: &[BigUint], n_i: &[BigUint]) -> BigUint {
    // calculating C = M^e mod (n1 ·n2 · ... ·nk),
    let n: BigUint = {
        let mut acc = BigUint::one();
        for x in n_i {
            acc *= x
        }
        acc
    };
    let c = solve_equations(
        &c_i.iter()
            .enumerate()
            .map(|(i, x)| ModuleEquation {
                a: x.clone(),
                n: n_i[i].clone(),
            })
            .collect(),
        &n,
    )
    .unwrap();

    c.nth_root(n_i.len() as u32)
}

pub fn perform_meet_in_the_middle_attack(
    l: u32,
    c: &BigUint,
    n: &BigUint,
) -> Result<BigUint, String> {
    let e = BigUint::from(E_65537);

    let mut x = Vec::new();
    for i in 1_u32..=(1 << (l >> 1)) {
        x.push((BigUint::from(i), BigUint::from(i).modpow(&e, n)))
    }

    for i in 0..=x.len() {
        let cs =
            (inverse(&BigInt::from(x[i].1.clone()), &BigInt::from(n.clone())).unwrap() * c) % n;
        for (t, t_e) in &x {
            if *t_e == cs && (*t_e != BigUint::one()) {
                println!(
                    "T: {:?}, S: {:?}",
                    t.to_str_radix(16),
                    x[i].0.to_str_radix(16)
                );
                return Ok(&x[i].0 * t);
            }
        }
    }

    Err("Unable to find value".to_string())
}

pub fn check_hastard_attack(c_i: &[BigUint], n_i: &[BigUint], m: &BigUint) -> Result<bool, String> {
    let e = BigUint::from(n_i.len());
    for i in 0..c_i.len() {
        if m.modpow(&e, &n_i[i]) != c_i[i] {
            return Err(format!("Failed to check i: {i} equation!!"));
        }
    }
    Ok(true)
}

pub fn check_meet_in_the_middle(c: &BigUint, n: &BigUint, m: &BigUint) -> Result<bool, String> {
    let e = BigUint::from(E_65537);
    if m.modpow(&e, &n) != *c {
        return Err(format!("Failed to check equation [m^e neq to C]!!"));
    }
    Ok(true)
}
