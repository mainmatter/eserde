// Wrapper that attaches context to a `Visitor`, `SeqAccess` or `EnumAccess`.
pub struct Wrap<X> {
    pub(crate) delegate: X,
}

// Wrapper that attaches context to a `VariantAccess`.
pub struct WrapVariant<X> {
    pub(crate) delegate: X,
    pub(crate) pop_path_segment_before_exit: bool,
}

impl<X> Wrap<X> {
    pub(crate) fn new(delegate: X) -> Self {
        Wrap { delegate }
    }
}

impl<X> WrapVariant<X> {
    pub(crate) fn new(delegate: X, pop_path_segment_before_exit: bool) -> Self {
        WrapVariant {
            delegate,
            pop_path_segment_before_exit,
        }
    }
}
