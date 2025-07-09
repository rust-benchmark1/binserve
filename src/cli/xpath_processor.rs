use xpath_reader::Reader;
use std::fs;

pub fn process_xpath_query(data: String) -> anyhow::Result<()> {
    let processed_query = build_xpath_expression(data);
    let xml_content = load_xml_document()?;
    let final_query = construct_user_query(processed_query);
    
    //SINK
    let reader = Reader::from_str(&xml_content, None)?;
    let _result: String = reader.read(&*final_query)?;
    Ok(())
}

// Transformer 1: Build XPath expression (doesn't sanitize)
fn build_xpath_expression(user_input: String) -> String {
    // Build XPath expression without escaping special characters
    let expression = format!("//user[username='{}']", user_input);
    expression
}

// Transformer 2: Load XML document (doesn't sanitize)
fn load_xml_document() -> anyhow::Result<String> {
    // Load XML content from file without validation
    let xml_content = fs::read_to_string("src/cli/users.xml")?;
    Ok(xml_content)
}

// Transformer 3: Construct user query (doesn't sanitize)
fn construct_user_query(query: String) -> String {
    // Construct final query without input validation
    let final_query = if query.contains("//user[") {
        query
    } else {
        format!("//user[username='{}']", query)
    };
    final_query
} 