//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-04, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("parallel"), DB CACHE: 128

// Executed Command:
// ./target/release/parallel
// benchmark
// --chain=parallel
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_multisig
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./runtime/parallel/src/weights//pallet_multisig.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_multisig.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
    fn as_multi_threshold_1(z: u32) -> Weight {
        (21_930_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as Weight))
    }
    fn as_multi_create(s: u32, z: u32) -> Weight {
        (67_258_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((222_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn as_multi_create_store(s: u32, z: u32) -> Weight {
        (78_192_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((205_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn as_multi_approve(s: u32, z: u32) -> Weight {
        (42_110_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((181_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn as_multi_approve_store(s: u32, z: u32) -> Weight {
        (74_474_000 as Weight)
            // Standard Error: 0
            .saturating_add((208_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn as_multi_complete(s: u32, z: u32) -> Weight {
        (97_713_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((369_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((5_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn approve_as_multi_create(s: u32) -> Weight {
        (69_550_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((196_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn approve_as_multi_approve(s: u32) -> Weight {
        (39_337_000 as Weight)
            // Standard Error: 0
            .saturating_add((193_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn approve_as_multi_complete(s: u32) -> Weight {
        (159_229_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((367_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn cancel_as_multi(s: u32) -> Weight {
        (116_949_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((219_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}
