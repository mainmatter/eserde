/// Implements [`crate::EDeserialize`] on the given type by falling back to `serde`'s default deserialization logic.
///
/// Equivalent to `#[eserde(compat)]` on a field.
///
/// This should only be used if `#[derive(eserde::Deserialize)]` is not possible,
/// e.g. for types that have a manual [`serde::Deserialize`] impl.
///
/// # Example
/// ```rust
/// use serde::de::{Deserialize, Deserializer};
/// use eserde::impl_edeserialize_compat;
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct U8OrU64(Result<u8, u64>);
/// impl<'de> Deserialize<'de> for U8OrU64 {
///     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
///         let n = u64::deserialize(deserializer)?;
///         Ok(U8OrU64(u8::try_from(n).map_err(|_| n)))
///     }
/// }
///
/// impl_edeserialize_compat!(U8OrU64);
///
/// # #[cfg(feature = "json")]
/// # {
/// assert_eq!(U8OrU64(Ok(42)), eserde::json::from_str::<U8OrU64>("42").unwrap());
/// assert_eq!(U8OrU64(Err(256)), eserde::json::from_str::<U8OrU64>("256").unwrap());
/// # }
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
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
}
impl_edeserialize_compat! {
    &'de [u8],
    &'de std::path::Path,
    &'de str,
    (),
    std::sync::atomic::AtomicBool,
    std::sync::atomic::AtomicI8,
    std::sync::atomic::AtomicI16,
    std::sync::atomic::AtomicI32,
    std::sync::atomic::AtomicI64,
    std::sync::atomic::AtomicIsize,
    std::sync::atomic::AtomicU8,
    std::sync::atomic::AtomicU16,
    std::sync::atomic::AtomicU32,
    std::sync::atomic::AtomicU64,
    std::sync::atomic::AtomicUsize,
    std::ops::Bound<T> {T},
    std::ffi::CString,
    std::time::Duration,
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
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

/// Implements [`crate::EDeserialize`] on the given type by falling back to `T`'s `EDeserialize` implementation.
///
/// This macro is not exported, it is for internal use only, analogous to `serde`'s internal `forwarded_impl!`.
///
/// Users should simply use `#[derive(EDeserialize)]` and `#[serde(transparent)]` on their single-field types.
macro_rules! impl_edeserialize_transparent {
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
            $g : $crate::EDeserialize<'de>,
            $( $( $bounds )* )?
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                <$g as $crate::EDeserialize>::deserialize_for_errors(deserializer)
            }
        }
        $( $crate::impl_edeserialize_transparent!( $( $rest )* ); )?
    };
}
pub(crate) use impl_edeserialize_transparent;

impl_edeserialize_transparent! {
    Box<T> { T where T: ?Sized },
    std::cell::Cell<T> { T where T: ?Sized },
    std::cell::RefCell<T> { T where T: ?Sized },
    std::cmp::Reverse<T> { T },
    std::rc::Rc<T> { T where T: ?Sized },
    std::rc::Weak<T> { T where T: ?Sized },
    std::sync::Arc<T> { T where T: ?Sized },
    std::sync::Mutex<T> { T where T: ?Sized },
    std::sync::RwLock<T> { T where T: ?Sized },
    std::sync::Weak<T> { T where T: ?Sized },
}

/// Implements [`crate::EDeserialize`] on the given sequence type by falling back to `T`'s `EDeserialize` implementation.
macro_rules! impl_edeserialize_seq {
    () => {};
    (
        $t:ty
        {
            $( $n:literal ; )?
            $g:ident $( , $h:ident ),* $(,)?
            $(where $( $bounds:tt )* )?
        }
        $(, $( $rest:tt )* )?
    ) => {
        impl<'de, $g $( , $h )* > $crate::EDeserialize<'de> for $t
        where
            Self: $crate::_serde::Deserialize<'de>,
            $g : $crate::EDeserialize<'de>,
            $( $( $bounds )* )?
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                // Wrapper which always succeeds but reports errors.
                struct Wrapper< $g >(::std::marker::PhantomData< $g >);
                impl<'de,  $g > $crate::_serde::de::Deserialize<'de> for Wrapper< $g >
                where
                    $g : $crate::EDeserialize<'de>,
                {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: $crate::_serde::Deserializer<'de>,
                    {
                        let _ =  $g ::deserialize_for_errors(deserializer);
                        Ok(Self(::std::marker::PhantomData))
                    }
                }

                let n_errors = $crate::reporter::ErrorReporter::n_errors();
                match <::std::vec::Vec<Wrapper< $g >> as $crate::_serde::Deserialize>::deserialize(deserializer) {
                    Err(err) => $crate::reporter::ErrorReporter::report(err),
                    Ok(vec) => {
                        $(
                            if vec.len() != $n {
                                $crate::reporter::ErrorReporter::report(::std::format!(
                                    "expected sequence of {} elements, found {} elements.",
                                    $n,
                                    vec.len(),
                                ));
                            }
                        )?
                        let _ = vec;
                    }
                }
                if $crate::reporter::ErrorReporter::n_errors() > n_errors {
                    Err(())
                } else {
                    Ok(())
                }
            }
        }
        $( $crate::impl_edeserialize_seq!( $( $rest )* ); )?
    };
}
pub(crate) use impl_edeserialize_seq;

// TODO: Sequences should collect the errors inside as well.
impl_edeserialize_seq! {
    [T; 0] {0; T},
    [T; 1] {1; T},
    [T; 2] {2; T},
    [T; 3] {3; T},
    [T; 4] {4; T},
    [T; 5] {5; T},
    [T; 6] {6; T},
    [T; 7] {7; T},
    [T; 8] {8; T},
    [T; 9] {9; T},
    [T; 10] {10; T},
    [T; 11] {11; T},
    [T; 12] {12; T},
    [T; 13] {13; T},
    [T; 14] {14; T},
    [T; 15] {15; T},
    [T; 16] {16; T},
    [T; 17] {17; T},
    [T; 18] {18; T},
    [T; 19] {19; T},
    [T; 20] {20; T},
    [T; 21] {21; T},
    [T; 22] {22; T},
    [T; 23] {23; T},
    [T; 24] {24; T},
    [T; 25] {25; T},
    [T; 26] {26; T},
    [T; 27] {27; T},
    [T; 28] {28; T},
    [T; 29] {29; T},
    [T; 30] {30; T},
    [T; 31] {31; T},
    [T; 32] {32; T},
    Box<[T]> {T},
    std::collections::BinaryHeap<T> {T},
    std::collections::BTreeSet<T> {T where T: std::cmp::Ord},
    std::collections::HashSet<T, S> {T, S where T: std::hash::Hash + std::cmp::Eq, S: std::hash::BuildHasher},
    std::collections::LinkedList<T> {T},
    std::collections::VecDeque<T> {T},
    Vec<T> {T},
}

/// Implements [`crate::EDeserialize`] on the given sequence type by falling back to `T`'s `EDeserialize` implementation.
macro_rules! impl_edeserialize_map {
    () => {};
    (
        $t:ty
        {
            $k:ident , $v:ident $( , $h:ident ),* $(,)?
            $(where $( $bounds:tt )* )?
        }
        $(, $( $rest:tt )* )?
    ) => {
        impl<'de, $k, $v $( , $h )* > $crate::EDeserialize<'de> for $t
        where
            Self: $crate::_serde::Deserialize<'de>,
            $k : $crate::EDeserialize<'de>,
            $v : $crate::EDeserialize<'de>,
            $( $( $bounds )* )?
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                // Wrapper which always succeeds but reports errors.
                struct Wrapper<T>(::std::marker::PhantomData<T>);
                impl<'de, T> $crate::_serde::de::Deserialize<'de> for Wrapper<T>
                where
                    T: $crate::EDeserialize<'de>,
                {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: $crate::_serde::Deserializer<'de>,
                    {
                        let _ = T::deserialize_for_errors(deserializer);
                        Ok(Self(::std::marker::PhantomData))
                    }
                }

                struct MapVisitor<K, V>(::std::marker::PhantomData<(K, V)>);
                impl <'de, K, V> $crate::_serde::de::Visitor<'de> for MapVisitor<K, V>
                where
                    K: $crate::EDeserialize<'de>,
                    V: $crate::EDeserialize<'de>,
                {
                    type Value = ();
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("map")
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: $crate::_serde::de::MapAccess<'de>,
                    {
                        while let Some((_, _)) = map.next_entry::<Wrapper<K>, Wrapper<V>>()? {}
                        Ok(())
                    }
                }

                let n_errors = $crate::reporter::ErrorReporter::n_errors();
                if let Err(err) = deserializer.deserialize_map(MapVisitor::<$k, $v>(::std::marker::PhantomData)) {
                    $crate::reporter::ErrorReporter::report(err);
                }
                if $crate::reporter::ErrorReporter::n_errors() > n_errors {
                    Err(())
                } else {
                    Ok(())
                }
            }
        }
        $( $crate::impl_edeserialize_map!( $( $rest )* ); )?
    };
}
pub(crate) use impl_edeserialize_map;

impl_edeserialize_map! {
    std::collections::BTreeMap<K, V> {K, V where K: std::cmp::Ord},
    std::collections::HashMap<K, V, S> {K, V, S where K: std::hash::Hash + std::cmp::Eq, S: std::hash::BuildHasher},
}

impl<'de, T> crate::EDeserialize<'de> for Option<T>
where
    T: crate::EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        struct OptionVisitor<T>(::std::marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for OptionVisitor<T>
        where
            T: crate::EDeserialize<'de>,
        {
            type Value = ();
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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
                let _ = T::deserialize_for_errors(deserializer);
                Ok(())
            }
        }

        let n_errors = crate::reporter::ErrorReporter::n_errors();
        if let Err(err) =
            deserializer.deserialize_option(OptionVisitor::<T>(std::marker::PhantomData))
        {
            crate::reporter::ErrorReporter::report(err);
        }
        if crate::reporter::ErrorReporter::n_errors() > n_errors {
            Err(())
        } else {
            Ok(())
        }
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
