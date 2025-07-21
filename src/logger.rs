
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_to_file(message: &str) {
    let path = "/tmp/my_streamworks.log";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
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
