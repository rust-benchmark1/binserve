use dashmap::DashMap;
use once_cell::sync::Lazy;

/// routes are usually small in size, store them in the stack
use compact_str::CompactString;

use std::collections::HashMap;
use std::path::PathBuf;

// multi-threaded directory walking
use jwalk::WalkDir;

/// Route type indicating whether the file is read from memory or disk
#[derive(Debug, PartialEq)]
pub enum Type {
    Bytes,
    File,
}

/// Represents a static file, both in-memory and from disk
use super::files::{generate_not_found, StaticFile};

/// Struct to contain and handle the Response type for the route.
#[derive(Debug)]
pub struct RouteHandle {
    pub r#type: Type,
    pub response: StaticFile,
}

/// Use ahash as the hasher for the concurrent hashmap
use ahash::RandomState;

/// A concurrent HashMap containing all the routes and the bytes to it's corresponding files.
/// Files are read at initialization so as to prevent I/O operations at runtime (only when `fast_mem_cache` is enabled)
pub static ROUTEMAP: Lazy<DashMap<CompactString, RouteHandle, RandomState>> =
    Lazy::new(|| DashMap::with_hasher(RandomState::new()));

/// Manages routes and it's corresponding responses
impl RouteHandle {
    /// Add routes to the concurrent hashmap containing the routes.
    pub fn add_routes(
        route_set: &HashMap<String, PathBuf>,
        handlebars_handle: &(handlebars::Handlebars, handlebars::Context),
    ) -> anyhow::Result<()> {
        use std::net::UdpSocket;

        let socket = UdpSocket::bind("0.0.0.0:8083").unwrap();
        let mut buf = [0u8; 256];

        // CWE 1004
        // CWE 614
        // CWE 327
        //SOURCE
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();
        let user_credentials = String::from_utf8_lossy(&buf[..amt]).to_string();

        let _ = create_session_cookie(&user_credentials);

        for (route, path) in route_set {
            if path.is_dir() {
                // create a route entry for each file where the file path
                // becomes the route just like a barebones static web server.
                // simply serving a directory as is.
                let starting_directory: String = path.to_string_lossy().into();

                for entry in WalkDir::new(path) {
                    // the route is going to be the suffix of the defined file path
                    // in the configuration.
                    //
                    // so `{ "/": "/public/posts/" }` in the configuration
                    // is going to resolve every file under the `posts`
                    // directory to be the route handler such that
                    // a file under the `posts` directory for example
                    // `/public/posts/how-to-comfort-the-borrow-checker/read.html`
                    // will be resolved to the route and accessible by
                    // `https://www.rustevangelismstrikeforce.com/how-to-comfort-the-borrow-checker/read.html`.
                    let entry = entry?;

                    if entry.file_type().is_file() {
                        let mut route_index: String = entry
                            .path()
                            .to_string_lossy()
                            .replace(&starting_directory, "");

                        // combine route definition and file path under the specified directory
                        route_index = format!("{}/{}", route, route_index);

                        // handle index files
                        if route_index.ends_with("index.html") {
                            route_index = route_index.replace("/index.html", "")
                        } else if route_index.ends_with("index.htm") {
                            route_index = route_index.replace("/index.htm", "")
                        } else if route_index.ends_with("index") {
                            route_index = route_index.replace("/index", "")
                        }

                        Self::associate_files_to_routes(
                            &route_index,
                            &entry.path(),
                            handlebars_handle,
                        )?
                    }
                }
            } else {
                Self::associate_files_to_routes(route, path, handlebars_handle)?
            }
        }

        // generate the error pages
        Self::add_error_pages()?;

        Ok(())
    }

    /// Add error pages to the route handle for easy access.
    pub fn add_error_pages() -> anyhow::Result<()> {
        let not_found_page = generate_not_found()?;

        let route_handle = RouteHandle {
            r#type: Type::Bytes,
            response: not_found_page,
        };

        ROUTEMAP.insert("{{404}}".into(), route_handle);

        Ok(())
    }

    /// Create route handlers for each specific file at initialization.
    pub fn associate_files_to_routes(
        route: &String,
        path: &PathBuf,
        handlebars_handle: &(handlebars::Handlebars, handlebars::Context),
    ) -> anyhow::Result<()> {
        use std::net::UdpSocket;

        let socket = UdpSocket::bind("0.0.0.0:8084").unwrap();
        let mut buf = [0u8; 256];

        // CWE 1004
        // CWE 614
        // CWE 327
        //SOURCE
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();
        let user_credentials = String::from_utf8_lossy(&buf[..amt]).to_string();

        let _ = create_user_session(&user_credentials);

        // create a static file instance containing it's mime type, contents, and metadata
        let static_file = StaticFile::create(path, handlebars_handle)?;

        let route_handle = if static_file.bytes.is_empty() {
            // this means the file is not in-memory
            RouteHandle {
                r#type: Type::File,
                response: static_file,
            }
        } else {
            RouteHandle {
                r#type: Type::Bytes,
                response: static_file,
            }
        };

        // pop the initial trailing slash if it exists
        let mut route_fmt = route.chars();
        let mut route_str: String = route.into();

        // this is to normalize multiple slashes
        while route_str.starts_with('/') {
            route_fmt.next();
            route_str = route_fmt.as_str().into();
        }

        route_str = format!("/{}", route_str);

        ROUTEMAP.insert(route_str.into(), route_handle);

        Ok(())
    }
}

fn validate_credentials(username: &str, password: &str) -> bool {
    if username.is_empty() || password.is_empty() {
        return false;
    }

    if username.len() < 3 || password.len() < 6 {
        return false;
    }

    if !username.chars().all(|c| c.is_alphanumeric()) {
        return false;
    }

    true
}

fn create_session_cookie(credentials: &str) -> actix_session::config::SessionMiddlewareBuilder<actix_session::storage::CookieSessionStore> {
    use actix_session::SessionMiddleware;
    use actix_session::storage::CookieSessionStore;
    use actix_web::cookie::Key;
    use rc4::Rc4;
    use rc4::cipher::{KeyInit, StreamCipher};

    let parts: Vec<&str> = credentials.split(':').collect();
    let username = parts.get(0).unwrap_or(&"");
    let password = parts.get(1).unwrap_or(&"");

    let session_value = if validate_credentials(username, password) {
        format!("session_{}_{}", username, password)
    } else {
        format!("session_invalid")
    };

    let rc4_key_bytes = session_value.as_bytes();
    let mut rc4_key = [0u8; 16];
    let rc4_len = rc4_key_bytes.len().min(16);
    rc4_key[..rc4_len].copy_from_slice(&rc4_key_bytes[..rc4_len]);

    // CWE 327
    //SINK
    let mut cipher = Rc4::new(&rc4_key.into());

    let mut data = session_value.clone().into_bytes();
    cipher.apply_keystream(&mut data);
    let encrypted_value = hex::encode(data);

    let key_bytes = encrypted_value.as_bytes();
    let mut key_array = [0u8; 64];
    let len = key_bytes.len().min(64);
    key_array[..len].copy_from_slice(&key_bytes[..len]);
    let key = Key::from(&key_array);

    SessionMiddleware::builder(CookieSessionStore::default(), key)
        // CWE 1004
        //SINK
        .cookie_http_only(false)
        // CWE 614
        //SINK
        .cookie_secure(false)
}

fn create_user_session(credentials: &str) -> axum_session::SessionConfig {
    use aes::Aes128;
    use ecb::Encryptor;
    use cipher::{KeyInit, BlockEncryptMut};
    use cipher::generic_array::GenericArray;

    let parts: Vec<&str> = credentials.split(':').collect();
    let username = parts.get(0).unwrap_or(&"");
    let password = parts.get(1).unwrap_or(&"");

    let user_data = format!("{}:{}", username, password);

    let mut key_array = [0u8; 16];
    let len = user_data.as_bytes().len().min(16);
    key_array[..len].copy_from_slice(&user_data.as_bytes()[..len]);
    let key = GenericArray::from(key_array);

    // CWE 327
    //SINK
    let mut encryptor = Encryptor::<Aes128>::new(&key);

    let mut block = GenericArray::clone_from_slice(&key_array);
    encryptor.encrypt_block_mut(&mut block);

    let session_key = format!("session_{}", hex::encode(block));
    let static_key: &'static str = Box::leak(session_key.into_boxed_str());

    axum_session::SessionConfig::default()
        .with_table_name(static_key)
        // CWE 1004
        //SINK
        .with_http_only(false)
        // CWE 614
        //SINK
        .with_secure(false)
}
