#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::enums_deny_unknown_fields::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(
        s,
        Struct,
        StructDenyUnknownFields,
        External,
        Internal,
        Adjacent
    );
});
