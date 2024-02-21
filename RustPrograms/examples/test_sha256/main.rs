// Copyright (c) 2021, COSIC-KU Leuven, Kasteelpark Arenberg 10, bus 2452, B-3001 Leuven-Heverlee, Belgium.
// Copyright (c) 2021, Cosmian Tech SAS, 53-55 rue La Boétie, Paris, France.

#![no_std]
#![no_main]
#![feature(const_evaluatable_checked, const_generics)]
#![allow(non_snake_case, incomplete_features)]

use scale_std::array::Array;

#[scale::main(KAPPA = 40)]
#[inline(always)]
fn main() {
    let mut state = Array::<SecretI64, 4>::uninitialized();
    state.set(3, &SecretI64::from(0x6a09e667bb67ae85u64 as i64));
    state.set(2, &SecretI64::from(0x3c6ef372a54ff53au64 as i64));
    state.set(1, &SecretI64::from(0x510e527f9b05688cu64 as i64));
    state.set(0, &SecretI64::from(0x1f83d9ab5be0cd19u64 as i64));

    let mut mess = Array::<SecretI64, 8>::uninitialized();
    mess.set(7, &SecretI64::from(0x8000000000000000u64 as i64));
    mess.set(6, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(5, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(4, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(3, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(2, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(1, &SecretI64::from(0x0000000000000000u64 as i64));
    mess.set(0, &SecretI64::from(0x0000000000000000u64 as i64));

    let out = scale_std::circuits::SHA256(mess, state).reveal();

    unsafe { out.get_unchecked(3) }.output(1);
    unsafe { out.get_unchecked(2) }.output(1);
    unsafe { out.get_unchecked(1) }.output(1);
    unsafe { out.get_unchecked(0) }.output(1);
}
