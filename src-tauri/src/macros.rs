#[macro_export]
///If the expression evaluates to an error, outputs it to the console before returning it
macro_rules! log_err {
    ($expr:expr) => {
        match $expr {
            Ok(v) => Ok(v),
            Err(e) => {
                eprintln!("[TAURI CMD ERROR] {}", e);
                Err(e)
            }
        }
    };
}
