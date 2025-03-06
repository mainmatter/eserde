/// Implements [`crate::EDeserialize`] on the given type by falling back to `serde`'s default deserialization logic.
///
/// Equivalent to `#[eserde(compat)]` on a field.
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
/// impl_edeserialize_compat!(MyStruct);
/// ```
#[macro_export]
macro_rules! impl_edeserialize_compat {
    () => {};
    (
        $t:ty
        $(
            {
                $( $g:ident ),* $(,)?
                $(where $( $bounds:tt )* )?
            }
        )?
        $(, $( $rest:tt )* )?
    ) => {
        impl<'de $( $( , $g )* )? > $crate::EDeserialize<'de> for $t
        where
            Self: $crate::_serde::Deserialize<'de>,
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
        $( $crate::impl_edeserialize_compat!( $( $rest )* ); )?
    };
}
impl_edeserialize_compat! {
    bool,
    char,
    f32,
    f64,
    i128,
    i16,
    i32,
    i64,
    i8,
    isize,
    u128,
    u16,
    u32,
    u64,
    u8,
    usize,
}
impl_edeserialize_compat! {
    &'de [u8],
    &'de std::path::Path,
    &'de str,
    (),
    std::sync::atomic::AtomicBool,
    std::sync::atomic::AtomicI16,
    std::sync::atomic::AtomicI32,
    std::sync::atomic::AtomicI64,
    std::sync::atomic::AtomicI8,
    std::sync::atomic::AtomicIsize,
    std::sync::atomic::AtomicU16,
    std::sync::atomic::AtomicU32,
    std::sync::atomic::AtomicU64,
    std::sync::atomic::AtomicU8,
    std::sync::atomic::AtomicUsize,
    std::collections::BTreeMap<K, V> {K, V where K: std::cmp::Ord},
    std::collections::BTreeSet<T> {T where T: std::cmp::Ord},
    std::collections::BinaryHeap<T> {T},
    std::ops::Bound<T> {T},
    std::ffi::CString,
    std::time::Duration,
    std::collections::HashMap<K, V, S> {K, V, S where K: std::hash::Hash + std::cmp::Eq, S: std::hash::BuildHasher},
    std::collections::HashSet<T, S> {T, S where T: std::hash::Hash + std::cmp::Eq, S: std::hash::BuildHasher},
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::collections::LinkedList<T> {T},
}
impl_edeserialize_compat! {
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
}
impl_edeserialize_compat! {
    Result<T, E> {T, E},
    std::collections::VecDeque<T> {T},
    std::ffi::OsString,
    std::marker::PhantomData<T> {T},
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
    std::num::Saturating<i128>,
    std::num::Saturating<i16>,
    std::num::Saturating<i32>,
    std::num::Saturating<i64>,
    std::num::Saturating<i8>,
    std::num::Saturating<isize>,
    std::num::Saturating<u128>,
    std::num::Saturating<u16>,
    std::num::Saturating<u32>,
    std::num::Saturating<u64>,
    std::num::Saturating<u8>,
    std::num::Saturating<usize>,
    std::num::Wrapping<T> {T},
    std::ops::Range<Idx> {Idx},
    std::ops::RangeFrom<Idx> {Idx},
    std::ops::RangeInclusive<Idx> {Idx},
    std::ops::RangeTo<Idx> {Idx},
    std::path::PathBuf,
    std::time::SystemTime,
    String,
}
impl_edeserialize_compat! {
    Vec<T> {T},
    [T; 0] {T},
    [T; 1] {T},
    [T; 2] {T},
    [T; 3] {T},
    [T; 4] {T},
    [T; 5] {T},
    [T; 6] {T},
    [T; 7] {T},
    [T; 8] {T},
    [T; 9] {T},
    [T; 10] {T},
    [T; 11] {T},
    [T; 12] {T},
    [T; 13] {T},
    [T; 14] {T},
    [T; 15] {T},
    [T; 16] {T},
    [T; 17] {T},
    [T; 18] {T},
    [T; 19] {T},
    [T; 20] {T},
    [T; 21] {T},
    [T; 22] {T},
    [T; 23] {T},
    [T; 24] {T},
    [T; 25] {T},
    [T; 26] {T},
    [T; 27] {T},
    [T; 28] {T},
    [T; 29] {T},
    [T; 30] {T},
    [T; 31] {T},
    [T; 32] {T},
}

// macro_rules! impl_edeserialize_seq {
//     () => {};
//     (
//         $t:ty
//         {
//             $g:ident $( , $h:ident )*
//             $(where $( $bounds:tt )* )?
//         }
//         $(, $( $rest:tt )* )?
//     ) => {
//         impl<'de, $g $( , $h )* > $crate::EDeserialize<'de> for $t
//         where
//             Self: $crate::_serde::Deserialize<'de>,
//             $g : $crate::EDeserialize<'de>,
//             $( $( $bounds )* )?
//         {
//             fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
//             where
//                 D: $crate::_serde::Deserializer<'de>,
//             {
//                 struct SeqVisitor< $g >(::td::marker::PhantomData< $g >);
//                 impl<'de, T $(, $typaram)*> Visitor<'de> for SeqVisitor<T $(, $typaram)*>
//                 where
//                     $g : $crate::EDeserialize<'de>,
//                 {
//                     type Value = ();
//                     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                         formatter.write_str("a sequence")
//                     }
//                     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//                     where
//                         A: SeqAccess<'de>,
//                     {
//                         let mut values = $with_capacity;
//                         while let Some(value) = tri!(seq.next_element()) {
//                             $insert(&mut values, value);
//                         }
//                         Ok(values)
//                     }
//                 }

//                 // < $g as $crate::EDeserialize>::deserialize_for_errors(deserializer)
//             }
//         }
//         $( $crate::impl_edeserialize_seq!( $( $rest )* ); )?
//     };
// }

/// Implements [`crate::EDeserialize`] on the given type by falling back to `T`'s `EDeserialize` implementation.
///
/// This macro is not exported, it is for internal use only, analogous to `serde`'s internal `forwarded_impl!`.
///
/// Users should simply use `#[derive(EDeserialize)]` and `#[serde(transparent)]` on their single-field types.
macro_rules! impl_edeserialize_forwarded {
    () => {};
    (
        $t:ty
        {
            $g:ident
            $(where $( $bounds:tt )* )?
        }
        $(, $( $rest:tt )* )?
    ) => {
        impl<'de, $g > $crate::EDeserialize<'de> for $t
        where
            Self: $crate::_serde::Deserialize<'de>,
            $g: $crate::EDeserialize<'de>,
            $( $( $bounds )* )?
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                <$g as $crate::EDeserialize>::deserialize_for_errors(deserializer)
            }
        }
        $( $crate::impl_edeserialize_forwarded!( $( $rest )* ); )?
    };
}
pub(crate) use impl_edeserialize_forwarded;

use crate::EDeserialize;
impl_edeserialize_forwarded! {
    Box<T> { T where T: ?Sized },
    std::cell::Cell<T> { T where T: ?Sized },
    std::cell::RefCell<T> { T where T: ?Sized },
    std::cmp::Reverse<T> { T },
    std::rc::Rc<T> { T where T: ?Sized },
    std::rc::Weak<T> { T where T: ?Sized },
    std::sync::Arc<T> { T where T: ?Sized },
    std::sync::Mutex<T> { T where T: ?Sized },
    std::sync::RwLock<T> { T where T: ?Sized },
}

impl<'de, T> crate::EDeserialize<'de> for Option<T>
where
    T: EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        struct OptionVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for OptionVisitor<T>
        where
            T: EDeserialize<'de>,
        {
            type Value = ();
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("option")
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(())
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(())
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize_for_errors(deserializer)?;
                Ok(())
            }
        }
        deserializer
            .deserialize_option(OptionVisitor::<T>(std::marker::PhantomData))
            .unwrap();
        Ok(())
    }
}

// Special case for Cow due to additional bounds
impl<'de, T> crate::EDeserialize<'de> for std::borrow::Cow<'_, T>
where
    T: ToOwned + ?Sized,
    T::Owned: crate::EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        T::Owned::deserialize_for_errors(deserializer)
    }
}
