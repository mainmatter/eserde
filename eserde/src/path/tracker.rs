use std::cell::RefCell;

use super::path::{Path, Segment};

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
        segments.map(|segments| Path { segments })
    }

    /// Stashes the current path for error handling.
    ///
    /// The replacement only takes place if the current path is not `None` and:
    ///
    /// - There is nothing stashed, or
    /// - The current path is not a strict subsequence of the currently stashed path
    pub fn stash_current_path_for_error() {
        LATEST_ERROR_PATH.with_borrow_mut(|stashed| {
            let current = Self::current_path();
            let Some(stashed) = stashed else {
                *stashed = current;
                return;
            };
            let Some(current) = current else {
                return;
            };
            if !stashed.segments.starts_with(&current.segments) {
                *stashed = current;
            }
        });
    }

    pub fn unstash_current_path_for_error() -> Option<Path> {
        LATEST_ERROR_PATH.take()
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
    static CURRENT_PATH: RefCell<Option<Vec<Segment>>> = RefCell::new(None);

    static LATEST_ERROR_PATH: RefCell<Option<Path>> = RefCell::new(None);
}
