use std::process::Command;

pub fn start_jupyter_notebook() {
    let output = Command::new("jupyter")
        .arg("notebook") // Optional: Pass additional arguments
        .output()
        .expect("Failed to execute command");

    // Print the output
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
    }

    // Print the error output, if any
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error:\n{}", stderr);
    }
}
