#[macro_export]
macro_rules! fuzz_many {
    ($s:expr, $($t:ty),+$(,)?) => {
        $({
            let _ = eserde_test_helper::test!(serialized; $t, $s).from_json();
        })+
    };
}
