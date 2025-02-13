use std::cell::RefCell;

use super::{Path, Segment};

pub struct PathTracker;

impl PathTracker {
    pub fn init() {
        CURRENT_PATH.with(|path| {
            if path.borrow().is_none() {
                *path.borrow_mut() = Some(Vec::new());
            }
        });
    }

    pub fn push(segment: Segment) {
        CURRENT_PATH.with_borrow_mut(|path| path.as_mut().map(|segments| segments.push(segment)));
    }

    pub fn pop() {
        CURRENT_PATH.with_borrow_mut(|path| {
            path.as_mut().map(|segments| {
                segments.pop();
            })
        });
    }

    pub fn current_path() -> Option<Path> {
        let segments = CURRENT_PATH.with_borrow(|segments| segments.clone());
        segments.map(|segments| segments.into())
    }

    /// Stashes the current path for error handling.
    ///
    /// The replacement only takes place if the current path is not `None` and:
    ///
    /// - There is nothing stashed, or
    /// - The current path is not a strict subsequence of the currently stashed path
    pub fn stash_current_path_for_error() {
        PATH_ON_ERROR.with_borrow_mut(|stashed| {
            let current = Self::current_path();
            let Some(stashed) = stashed else {
                *stashed = current;
                return;
            };
            let Some(current) = current else {
                return;
            };
            if !stashed.segments().starts_with(current.segments()) {
                *stashed = current;
            }
        });
    }

    pub fn unstash_current_path_for_error() -> Option<Path> {
        PATH_ON_ERROR.take()
    }

    pub fn try_unset() {
        let _ = CURRENT_PATH.try_with(|s| {
            if let Ok(mut s) = s.try_borrow_mut() {
                *s = None;
            }
        });
    }
}

thread_local! {
    /// The path to the value we're currently trying to deserialize.
    static CURRENT_PATH: RefCell<Option<Vec<Segment>>> = const { RefCell::new(None) };

    /// A snapshot of the current path, captured when an error occurred.
    ///
    /// For types that implement [`EDeserialize`], this is not necessary
    /// since we manually capture the current path when reporting the error.
    ///
    /// That's not the case for types that only implement `serde`'s Serialize
    /// though. In those cases, we need to save the current path here
    /// in order to retrieve it later when we get back into `eserde`'s territory
    /// and try to report the error.
    static PATH_ON_ERROR: RefCell<Option<Path>> = const { RefCell::new(None) };
}
