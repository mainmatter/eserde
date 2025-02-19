#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::{contract::*, json::JsonValue};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: JsonValue| {
    let s = serde_json::to_string(&s).unwrap();
    fuzz_many!(
        &s,
        StructDenyUnknownFields,
        StructAllowUnknownFields,
        TupleStruct,
        ExternalEnum,
        InternalEnum,
        AdjacentEnum,
    );
});
