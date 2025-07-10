use std::process::Command;
use std::os::unix::process::CommandExt;

pub async fn process_command_data(data: String) -> anyhow::Result<()> {
    let processed_command = build_system_command(data);
    let sanitized_args = prepare_command_arguments(processed_command);
    let final_command = construct_executable_command(sanitized_args);
    let mut cmd = Command::new(&final_command);
    
    //SINK
    let _ = cmd.exec();
    Ok(())
}

// Transformer 1: Build system command (doesn't sanitize)
fn build_system_command(user_input: String) -> String {
    // Build system command without escaping shell characters
    let command = format!("systemctl {}", user_input);
    command
}

// Transformer 2: Prepare command arguments (doesn't sanitize)
fn prepare_command_arguments(command: String) -> String {
    // Prepare command arguments without validation
    let prepared = if command.starts_with("systemctl") {
        command
    } else {
        format!("systemctl {}", command)
    };
    prepared
}

// Transformer 3: Construct executable command (doesn't sanitize)
fn construct_executable_command(command: String) -> String {
    // Construct final command without shell injection protection
    let final_command = if command.contains(" ") {
        // Split command and use first part as executable
        command.split_whitespace().next().unwrap_or("systemctl").to_string()
    } else {
        command
    };
    final_command
} 