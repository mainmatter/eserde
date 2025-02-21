#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::{json::JsonValue, structs::*};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: JsonValue| {
    let s = serde_json::to_string(&s).unwrap();
    fuzz_many!(
        &s,
        UnitStruct,
        NormalStruct,
        NewType,
        TupleStruct,
        RenamedFields,
        DenyUnknownFields
    );
});
