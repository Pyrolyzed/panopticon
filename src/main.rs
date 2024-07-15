use std::process::Command;

fn run_command(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                eprintln!("Error executing command: {}", command);
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                String::new()
            }
        },
        Err(err) => {
            eprintln!("Error executing command: {}", err);
            String::new()
        }
    }
}

fn main() {

}