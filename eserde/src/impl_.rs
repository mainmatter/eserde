/// Implements `EDeserialize` for the given type by delegating to `serde::Deserialize` and only reporting one error.
///
/// # Example
/// ```rust
/// use serde::Deserialize;
/// use eserde::impl_edeserialize;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     field: i32,
/// }
///
/// impl_edeserialize!(MyStruct);
/// ```
#[macro_export]
macro_rules! impl_edeserialize {
    () => {};
    (
        $t:ty
        $(
            [
                $( $g:ident ),* $(,)?
                $(where $( $bounds:tt )* )?
            ]
        )?
        $(, $( $rest:tt )* )?
    ) => {
        impl<'de $( $( , $g )* )? > $crate::EDeserialize<'de> for $t
        where
            Self: $crate::_serde::Deserialize<'de>,
            // $(
            //     $( $g : $crate::EDeserialize<'de>, )*
            // )?
            $( $( $( $bounds )* )? )?
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                <Self as $crate::_serde::Deserialize>::deserialize(deserializer).map(|_| ()).map_err(|e| {
                    $crate::reporter::ErrorReporter::report(e);
                })
            }
        }
        $( $crate::impl_edeserialize!( $( $rest )* ); )?
    };
    (
        $t:ty
        $(, $( $rest:tt )* )?
    ) => {
        $crate::impl_edeserialize!(
            (
                <> $t
            )
        );
        $( $crate::impl_::impl_edeserialize!( $( $rest )* ); )?
    };
}

impl_edeserialize! {
    (),
    (T,) [T],
    &'de [u8],
    &'de std::path::Path,
    &'de str,
    bool,
    Box<[T]> [T],
    Box<std::ffi::CStr>,
    Box<std::ffi::OsStr>,
    Box<std::path::Path>,
    Box<str>,
    Box<T> [T],
    char,
    f32,
    f64,
    i128,
    i16,
    i32,
    i64,
    i8,
    isize,
    Option<T> [T],
    std::borrow::Cow<'_, str>,
    std::cell::Cell<T> [T],
    std::cell::RefCell<T> [T],
    std::cmp::Reverse<T> [T],
    std::collections::BinaryHeap<T> [T where T: std::cmp::Ord],
    std::collections::BTreeMap<K, V> [K, V where K: std::cmp::Ord],
    std::collections::BTreeSet<T> [T where T: std::cmp::Ord],
    std::collections::HashMap<K, V> [K, V where K: std::hash::Hash + std::cmp::Eq],
    std::collections::HashSet<T> [T where T: std::hash::Hash + std::cmp::Eq],
    std::collections::LinkedList<T> [T],
    std::collections::VecDeque<T> [T],
    std::num::NonZeroI128,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI8,
    std::num::NonZeroIsize,
    std::num::NonZeroU128,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU8,
    std::num::NonZeroUsize,
    std::ops::Bound<T> [T],
    std::ops::Range<T> [T],
    std::ops::RangeFrom<T> [T],
    std::ops::RangeInclusive<T> [T],
    std::ops::RangeTo<T> [T],
    String,
    u128,
    u16,
    u32,
    u64,
    u8,
    usize,
    Vec<T> [T],
    std::time::Duration,
    std::time::SystemTime,
    std::time::Instant,
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::sync::Mutex<T> [T],
    std::sync::RwLock<T> [T],
    std::ffi::OsString,
    std::path::PathBuf,
    std::marker::PhantomData<T> [T],
}

struct NotSerde;
impl_edeserialize!(NotSerde);
