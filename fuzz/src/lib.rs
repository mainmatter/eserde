#[macro_export]
macro_rules! fuzz_many {
    ($s:expr, $($t:ty),+$(,)?) => {
        $({
            let _ = eserde_test_helper::test_helper::TestHelper::<$t>::new_serialized($s).from_json();
        })+
    };
}
