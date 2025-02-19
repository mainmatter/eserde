#![no_main]
#![allow(dead_code)]

use eserde_test_helper::structs::*;
use libfuzzer_sys::fuzz_target;
use eserde_fuzz::fuzz_many;

fuzz_target!(|s: &str| {
    fuzz_many!(s, UnitStruct, NormalStruct, NewType, TupleStruct, RenamedFields, DenyUnknownFields);
});
