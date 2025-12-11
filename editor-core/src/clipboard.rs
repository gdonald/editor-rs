use crate::error::Result;

#[cfg(not(test))]
use crate::error::EditorError;

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

#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
thread_local! {
    #[allow(clippy::missing_const_for_thread_local)]
    static TEST_CLIPBOARD: RefCell<Option<String>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    #[doc(hidden)]
    pub fn clear_test_clipboard() {
        #[cfg(test)]
        {
            TEST_CLIPBOARD.with(|clipboard| {
                *clipboard.borrow_mut() = None;
            });
        }
    }

    #[cfg(not(test))]
    pub fn set_text(&self, text: &str) -> Result<()> {
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
        TEST_CLIPBOARD.with(|clipboard| {
            *clipboard.borrow_mut() = Some(text.to_string());
        });
        Ok(())
    }

    #[cfg(not(test))]
    pub fn get_text(&self) -> Result<String> {
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
        TEST_CLIPBOARD.with(|clipboard| Ok(clipboard.borrow().clone().unwrap_or_default()))
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self
    }
}
