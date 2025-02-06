use std::cell::RefCell;

use crate::DeserializationError;

/// The entrypoint for reporting errors that occurred during [`HumanDeserialize::human_deserialize`](crate::HumanDeserialize::human_deserialize).
///
/// Check out [`ErrorReporter::start_deserialization`] for more information.
pub struct ErrorReporter;

impl ErrorReporter {
    #[must_use = "The guard returned by this method must be kept alive for the duration of the whole \
        deserialization operation to ensure that errors are correctly reported."]
    /// Kick-off a deserialization operation.
    ///
    /// This method must be invoked before calling [`HumanDeserialize::human_deserialize`](crate::HumanDeserialize::human_deserialize),
    /// otherwise the deserializer will panic when trying to report errors.
    ///
    /// The returned guard must be kept alive for the duration of the whole deserialization operation.
    /// Once the guard is dropped, the deserialization operation is considered finished and the accumulated
    /// errors are cleared.
    ///
    /// In most cases, you don't need to call this method directly, as it's usually taken care of by the
    /// format-specific functions provided by `eserde`, such as [`eserde::json::from_str`](crate::json::from_str).
    pub fn start_deserialization() -> ErrorReporterGuard {
        DESERIALIZATION_ERRORS.set(Some(Vec::new()));
        ErrorReporterGuard
    }

    /// Report an error that occurred during deserialization.
    ///
    /// # Panics
    ///
    /// This method will panic if called outside of a deserialization operation.
    /// Check out [`ErrorReporter::start_deserialization`] for more information.
    pub fn report(error: DeserializationError) {
        let success = DESERIALIZATION_ERRORS.with_borrow_mut(|v| {
            if let Some(v) = v {
                v.push(error);
                true
            } else {
                false
            }
        });
        if !success {
            // TODO: Should we panic directly inside the `with_borrow_mut` closure?
            //   Or does that have some weird side-effects?
            panic!("Attempted to report an error outside of a deserialization operation. \
                You can't call `ErrorReporter::report_error` without first calling `ErrorReporter::start_deserialization`. \
                This error may be triggered by a top-level invocation of `HumanDeserialize::human_deserialize` without \
                a preceding call to `ErrorReporter::start_deserialization`. \
                This initialization step is usually taken care of by the format-specific functions provided by `eserde`, \
                such as `eserde::json::from_str`. If you're implementing your own deserialization logic, you \
                need to take care of this initialization step yourself.");
        };
    }

    /// Retrieve all errors that occurred during deserialization up to this point.
    ///
    /// The buffer is cleared after this call—i.e. subsequent calls to this method will return
    /// an empty vector until new errors are reported.
    ///
    /// # Panics
    ///
    /// This method will panic if called outside of a deserialization operation.
    /// Check out [`ErrorReporter::start_deserialization`] for more information.
    pub fn take_errors() -> Vec<DeserializationError> {
        DESERIALIZATION_ERRORS.with_borrow_mut(|v| v.replace(Vec::new()))
            .expect(
                "Attempted to collect deserialization errors outside of a deserialization operation. \
                You can't call `ErrorReporter::take_errors` without first calling `ErrorReporter::start_deserialization`. \
                This error may be triggered by a top-level invocation of `HumanDeserialize::human_deserialize` without \
                a preceding call to `ErrorReporter::start_deserialization`. \
                This initialization step is usually taken care of by the format-specific functions provided by `eserde`, \
                such as `eserde::json::from_str`. If you're implementing your own deserialization logic, you \
                need to take care of this initialization step yourself.")
    }

    /// Retrieve the number of errors that occurred during deserialization up to this point.
    ///
    /// # Panics
    ///
    /// This method will panic if called outside of a deserialization operation.
    /// Check out [`ErrorReporter::start_deserialization`] for more information.
    pub fn n_errors() -> usize {
        DESERIALIZATION_ERRORS.with_borrow(|v| v.as_ref().map(|v| v.len()))
            .expect(
                "Attempted to count the number of deserialization errors outside of a deserialization operation. \
                You can't call `ErrorReporter::take_errors` without first calling `ErrorReporter::start_deserialization`. \
                This error may be triggered by a top-level invocation of `HumanDeserialize::human_deserialize` without \
                a preceding call to `ErrorReporter::start_deserialization`. \
                This initialization step is usually taken care of by the format-specific functions provided by `eserde`, \
                such as `eserde::json::from_str`. If you're implementing your own deserialization logic, you \
                need to take care of this initialization step yourself.")
    }
}

#[non_exhaustive]
/// Guard returned by [`ErrorReporter::start_deserialization`].
///
/// As long as this guard is alive, you're within the context of a single deserialization operation.
/// All errors that occur during deserialization (either of a top-level value or of a nested value)
/// will be accumulated in a single buffer, which is then retrieved at the end of the operation.
///
/// Once this guard is dropped, the buffer is cleared and the deserialize operation is considered
/// finished.
pub struct ErrorReporterGuard;

impl Drop for ErrorReporterGuard {
    fn drop(&mut self) {
        let _ = DESERIALIZATION_ERRORS.try_with(|c| {
            if let Ok(mut v) = c.try_borrow_mut() {
                *v = None;
            }
        });
    }
}

thread_local! {
    /// Errors that occurred during deserialization.
    ///
    /// # Why a thread-local?
    ///
    /// We use a thread-local since we are constrained by the signature of `serde`'s `Deserialize`
    /// trait, so we can't pass down a `&mut Vec<_>` to accumulate errors.
    static DESERIALIZATION_ERRORS: RefCell<Option<Vec<DeserializationError>>> = RefCell::new(None);
}
