#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::contract::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(
        s,
        StructDenyUnknownFields,
        StructAllowUnknownFields,
        TupleStruct,
        ExternalEnum,
        InternalEnum,
        AdjacentEnum,
    );
});
