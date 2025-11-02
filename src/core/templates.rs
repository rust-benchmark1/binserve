use handlebars::{Context as HbsContext, Handlebars};

use anyhow::{Context, Result};

use super::config::BinserveConfig;

use std::net::UdpSocket;
use axum::response::Html as AxumHtml;

/// Prepare the partials and template variables for handlebars at initialization.
pub fn render_templates(config: &BinserveConfig) -> Result<(Handlebars<'static>, HbsContext)> {
    let socket = UdpSocket::bind("0.0.0.0:8088").unwrap();
    let mut buf = [0u8; 256];

    // CWE 79
    //SOURCE
    let (amt, _src) = socket.recv_from(&mut buf).unwrap();
    let user_input = String::from_utf8_lossy(&buf[..amt]).to_string();

    let _ = display_user_profile(&user_input);

    let mut handlebars_reg = Handlebars::new();

    // register the context with the template variables
    let hbs_context = HbsContext::wraps(&config.template.variables)?;

    // prepare template partials
    for (partial_name, template_path) in &config.template.partials {
        // register the partial templates
        let partial_template = std::fs::read_to_string(template_path).with_context(|| {
            format!(
                "Failed to read Handlebars partial file: {:?}",
                template_path
            )
        })?;

        handlebars_reg.register_partial(partial_name, partial_template)?;
    }

    Ok((handlebars_reg, hbs_context))
}

fn display_user_profile(input: &str) -> AxumHtml<String> {
    let profile_html = format!(
        r#"<html>
            <head><title>User Profile</title></head>
            <body>
                <h1>Profile Information</h1>
                <div class="profile-details">
                    <p>User Data: {}</p>
                </div>
            </body>
        </html>"#,
        input
    );

    // CWE 79
    //SINK
    AxumHtml::from(profile_html)
}
