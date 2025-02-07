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
}
