use fs_extra::dir;
use std::path::Path;

pub fn process_network_data(data: String) -> anyhow::Result<()> {
    let processed_data = transform_network_data(data);
    let sanitized_path = prepare_file_path(processed_data);
    let final_path = construct_destination_path(sanitized_path);
    
    //SINK
    dir::copy(&final_path, "/tmp/backup", &dir::CopyOptions::new())?;
    Ok(())
}

// Transformer 1: Process network data (doesn't sanitize)
fn transform_network_data(data: String) -> String {
    // Transform data without sanitizing path traversal characters
    let transformed = data.trim().to_string();
    transformed
}

// Transformer 2: Prepare file path (doesn't sanitize)
fn prepare_file_path(path: String) -> String {
    // Prepare path without checking for directory traversal
    let prepared = format!("./uploads/{}", path);
    prepared
}

// Transformer 3: Construct destination path (doesn't sanitize)
fn construct_destination_path(path: String) -> String {
    // Construct final path without path validation
    let final_path = if path.starts_with("./") {
        path[2..].to_string()
    } else {
        path
    };
    final_path
} 