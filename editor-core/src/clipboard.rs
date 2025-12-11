use crate::error::Result;

#[cfg(not(test))]
use crate::error::EditorError;

#[cfg(not(test))]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(test))]
use arboard::Clipboard;
#[cfg(not(test))]
use parking_lot::Mutex;
#[cfg(not(test))]
use std::sync::Arc;

#[cfg(not(test))]
static GLOBAL_CLIPBOARD: once_cell::sync::Lazy<Arc<Mutex<Option<Clipboard>>>> =
    once_cell::sync::Lazy::new(|| {
        let clipboard = Clipboard::new().ok();
        Arc::new(Mutex::new(clipboard))
    });

#[cfg(not(test))]
static USE_MOCK: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
static MOCK_CLIPBOARD: once_cell::sync::Lazy<Arc<Mutex<Option<String>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
static TEST_CLIPBOARD: once_cell::sync::Lazy<Arc<Mutex<Option<String>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Clone)]
pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    #[doc(hidden)]
    pub fn enable_mock_clipboard() {
        #[cfg(not(test))]
        USE_MOCK.store(true, Ordering::SeqCst);
    }

    #[doc(hidden)]
    pub fn clear_test_clipboard() {
        #[cfg(test)]
        {
            *TEST_CLIPBOARD.lock().unwrap() = None;
        }
        #[cfg(not(test))]
        {
            if USE_MOCK.load(Ordering::SeqCst) {
                *MOCK_CLIPBOARD.lock() = None;
            }
        }
    }

    #[cfg(not(test))]
    pub fn set_text(&self, text: &str) -> Result<()> {
        if USE_MOCK.load(Ordering::SeqCst) {
            *MOCK_CLIPBOARD.lock() = Some(text.to_string());
            return Ok(());
        }

        let mut clipboard = GLOBAL_CLIPBOARD.lock();
        if let Some(cb) = clipboard.as_mut() {
            cb.set_text(text).map_err(|e| {
                EditorError::InvalidOperation(format!("Failed to set clipboard text: {}", e))
            })?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn set_text(&self, text: &str) -> Result<()> {
        *TEST_CLIPBOARD.lock().unwrap() = Some(text.to_string());
        Ok(())
    }

    #[cfg(not(test))]
    pub fn get_text(&self) -> Result<String> {
        if USE_MOCK.load(Ordering::SeqCst) {
            let clipboard = MOCK_CLIPBOARD.lock();
            return Ok(match &*clipboard {
                Some(text) => text.clone(),
                None => String::new(),
            });
        }

        let mut clipboard = GLOBAL_CLIPBOARD.lock();
        if let Some(cb) = clipboard.as_mut() {
            cb.get_text().map_err(|e| {
                EditorError::InvalidOperation(format!("Failed to get clipboard text: {}", e))
            })
        } else {
            Ok(String::new())
        }
    }

    #[cfg(test)]
    pub fn get_text(&self) -> Result<String> {
        let clipboard = TEST_CLIPBOARD.lock().unwrap();
        Ok(match &*clipboard {
            Some(text) => text.clone(),
            None => String::new(),
        })
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self
    }
}
