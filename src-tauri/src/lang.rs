//! Interface language for backend messages and generated explanations. The frontend
//! sets it from the display settings; Indonesian is the default.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

static ENGLISH: AtomicBool = AtomicBool::new(false);

thread_local! {
    /// Per-thread override, set by `with_english` (report exports, tests).
    static OVERRIDE: Cell<Option<bool>> = const { Cell::new(None) };
}

pub fn set_english(on: bool) {
    ENGLISH.store(on, Ordering::Relaxed);
}

pub fn english() -> bool {
    OVERRIDE.with(Cell::get).unwrap_or_else(|| ENGLISH.load(Ordering::Relaxed))
}

/// Runs `f` with the language fixed for this thread only, e.g. so a report written in
/// one language gets generated gap titles in that language whatever the interface uses.
pub fn with_english<T>(on: bool, f: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            OVERRIDE.with(|o| o.set(self.0));
        }
    }
    let _restore = Restore(OVERRIDE.with(|o| o.replace(Some(on))));
    f()
}

/// `tr!("Indonesia", "English")`: picks the text for the current language. Each side is
/// evaluated only when chosen, so both may be `format!` calls.
#[macro_export]
macro_rules! tr {
    ($id:expr, $en:expr $(,)?) => {
        if $crate::lang::english() {
            $en
        } else {
            $id
        }
    };
}

/// Sets the language of backend messages: "id" or "en".
#[tauri::command]
pub fn set_language(language: String) -> crate::error::AppResult<()> {
    match language.as_str() {
        "id" | "en" => {
            set_english(language == "en");
            Ok(())
        }
        _ => Err(crate::error::AppError::Invalid(tr!("bahasa tidak dikenal", "unknown language").into())),
    }
}

#[cfg(test)]
mod tests {
    use super::with_english;

    #[test]
    fn override_is_scoped_to_the_call() {
        assert_eq!(with_english(true, || crate::tr!("celah", "gap")), "gap");
        assert_eq!(with_english(true, || with_english(false, || crate::tr!("celah", "gap"))), "celah");
        assert_eq!(with_english(true, || crate::db::stats::label("Referensi")), "References");
        assert_eq!(crate::db::stats::label("Referensi"), "Referensi");
    }
}
