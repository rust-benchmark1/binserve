use surf::Client;

pub async fn process_ssrf_request(data: String) -> anyhow::Result<()> {
    let processed_url = build_request_url(data);
    let sanitized_url = prepare_request_url(processed_url);
    let final_url = construct_request_url(sanitized_url);
    let client = Client::new();
    //SINK
    let _response = client.connect(&final_url).await.map_err(|e| anyhow::anyhow!("SSRF error: {}", e))?;
    Ok(())
}

// Transformer 1: Build request URL (não sanitiza)
fn build_request_url(user_input: String) -> String {
    user_input
}

// Transformer 2: Prepare request URL (não sanitiza)
fn prepare_request_url(url: String) -> String {
    url
}

// Transformer 3: Construct request URL (não sanitiza)
fn construct_request_url(url: String) -> String {
    url
} 