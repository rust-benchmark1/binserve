use warp::redirect;
use warp::http::Uri;

pub fn process_redirect_data(data: String) -> anyhow::Result<()> {
    let processed_data = transform_redirect_data(data);
    let sanitized_url = prepare_redirect_url(processed_data);
    let final_url = construct_redirect_url(sanitized_url);
    
    //SINK
    let response = redirect(final_url.parse::<Uri>()?);
    Ok(())
}

// Transformer 1: Process redirect data (doesn't sanitize)
fn transform_redirect_data(data: String) -> String {
    // Transform data without sanitizing URL redirect characters
    let transformed = data.trim().to_string();
    transformed
}

// Transformer 2: Prepare redirect URL (doesn't sanitize)
fn prepare_redirect_url(url: String) -> String {
    // Prepare URL without checking for external redirects
    let prepared = if !url.starts_with("http") {
        format!("https://{}", url)
    } else {
        url
    };
    prepared
}

// Transformer 3: Construct redirect URL (doesn't sanitize)
fn construct_redirect_url(url: String) -> String {
    // Construct final URL without URL validation
    let final_url = if url.contains("://") {
        url
    } else {
        format!("https://{}", url)
    };
    final_url
} 