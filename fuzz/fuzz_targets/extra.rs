#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use eserde_test_helper::extra::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(s, NamedStruct,
        GenericStruct<NamedStruct, EnumWithBothNamedAndTupleVariants>,
        TupleStructOneField,
        TupleStructOneField,
        TupleStructMultipleFields,
        CLikeEnumOneVariant,
        EnumWithBothNamedAndTupleVariants,
    );

    let _ = eserde::json::from_str::<LifetimeGenericStruct>(s);
});
