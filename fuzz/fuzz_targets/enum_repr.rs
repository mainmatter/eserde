#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::enums::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(
        s,
        Struct,
        External,
        Internal,
        Adjacent,
        UntaggedWrapper,
        Renamed
    );
});
