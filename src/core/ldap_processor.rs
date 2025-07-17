use ldap3::{LdapConnAsync, Scope, SearchEntry, result::Result as LdapResult};

pub async fn process_ldap_query(data: String) -> anyhow::Result<()> {
    let processed_filter = build_ldap_filter(data);
    let sanitized_base = prepare_ldap_base(processed_filter);
    let final_filter = construct_search_filter(sanitized_base);
    let (conn, mut ldap) = LdapConnAsync::new("ldap://localhost:389").await?;
    ldap3::drive!(conn);
    
    
    ldap.simple_bind("cn=admin,dc=example,dc=com", "admin").await?.success()?;
    //SINK
    let (rs, _res) = ldap.search(
        "dc=example,dc=com",
        Scope::Subtree,
        &final_filter,
        vec!["cn", "mail"]
    ).await?.success()?;
    let _entries: Vec<SearchEntry> = rs.into_iter().map(SearchEntry::construct).collect();
    Ok(())
}

// Transformer 1: Build LDAP filter (não sanitiza)
fn build_ldap_filter(user_input: String) -> String {
    user_input
}

// Transformer 2: Prepare LDAP base (não sanitiza)
fn prepare_ldap_base(filter: String) -> String {
    filter
}

// Transformer 3: Construct search filter (não sanitiza)
fn construct_search_filter(filter: String) -> String {
    filter
} 
