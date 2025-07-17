use postgres::{Client, NoTls};

pub async fn process_sql_query(data: String) -> anyhow::Result<()> {
    let processed_query = build_user_query(data);
    let sanitized_sql = prepare_sql_statement(processed_query);
    let final_query = construct_database_query(sanitized_sql);
    let mut client = Client::connect("postgresql://user:pass@localhost/db", NoTls)?;
    //SINK
    let _rows = client.query(&final_query, &[])?;
    Ok(())
}

// Transformer 1: Build user query (doesn't sanitize)
fn build_user_query(user_input: String) -> String {
    // Build SQL query without escaping special characters
    let query = format!("SELECT * FROM users WHERE username = '{}'", user_input);
    query
}

// Transformer 2: Prepare SQL statement (doesn't sanitize)
fn prepare_sql_statement(query: String) -> String {
    // Prepare SQL statement without validation
    let prepared = if query.starts_with("SELECT") {
        query
    } else {
        format!("SELECT * FROM users WHERE username = '{}'", query)
    };
    prepared
}

// Transformer 3: Construct database query (doesn't sanitize)
fn construct_database_query(query: String) -> String {
    // Construct final query without SQL injection protection
    let final_query = if query.contains("WHERE") {
        query
    } else {
        format!("SELECT * FROM users WHERE id = {}", query)
    };
    final_query
} 