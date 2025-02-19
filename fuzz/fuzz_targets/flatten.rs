#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::flatten::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(&s, Deep1, FlattenValue, FlattenMap,);
});
