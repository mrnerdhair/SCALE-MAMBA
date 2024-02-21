// Copyright (c) 2021, COSIC-KU Leuven, Kasteelpark Arenberg 10, bus 2452, B-3001 Leuven-Heverlee, Belgium.
// Copyright (c) 2021, Cosmian Tech SAS, 53-55 rue La Boétie, Paris, France.

#![no_std]
#![no_main]
#![feature(const_evaluatable_checked, const_generics)]
#![allow(non_snake_case, incomplete_features)]

use scale_std::bit_protocols::{KOpL, KOr};
use scale_std::fixed_point::*;
use scale_std::integer::*;
use scale_std::math::*;
use scale_std::slice::Slice;
use scale_std::array::Array;

#[scale::main(KAPPA = 40)]
#[inline(always)]
fn main() {
    let state = Array::<SecretI64, 4>::from_iter([
        SecretI64::from(0x6a09e667bb67ae85),
        SecretI64::from(0x3c6ef372a54ff53a),
        SecretI64::from(0x510e527f9b05688c),
        SecretI64::from(0x1f83d9ab5be0cd19),
    ].into_iter());

    let mess = Array::<SecretI64, 8>::from_iter([
        SecretI64::from(0x8000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
        SecretI64::from(0x0000000000000000),
    ].into_iter());

    let out = scale_std::circuits::SHA256(mess, state).reveal();

    println!("{:08x}{:08x}{:08x}{:08x}", out.get(0).unwrap(), out.get(1).unwrap(), out.get(2).unwrap(), out.get(3).unwrap());
}
