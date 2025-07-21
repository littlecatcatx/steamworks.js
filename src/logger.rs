
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

static LOG_INIT: Once = Once::new();

pub fn log_to_file(message: &str) {
    let mut path = env::temp_dir();
    path.push("my_streamworks.log");

    LOG_INIT.call_once(|| {
        // This block runs only once per process
        // Attempt to remove the old log file, ignoring errors if it doesn't exist
        let _ = fs::remove_file(&path);
        // Print the log path to the console for easy access
        println!("[steamworks.js] logging to: {}", path.display());
    });

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();

    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_millis();

    if let Err(e) = writeln!(file, "[{}] {}", timestamp, message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn log_function_call(function_name: &str, args: &[&dyn std::fmt::Debug], result: &dyn std::fmt::Debug) {
    let args_str = args
        .iter()
        .map(|arg| format!("{:?}", arg))
        .collect::<Vec<String>>()
        .join(", ");
    log_to_file(&format!(
        "Called {} with args: ({}) and got result: {:?}",
        function_name, args_str, result
    ));
}
