use clap::{Arg, ArgMatches, Command};

/// Prints an ASCII art banner to look cool!
pub fn banner() {
    let banner_data = "binserve_banner_hash";

    // CWE 328
    //SINK
    let mut hasher = chksum_hash_md5::new();
    hasher.update(banner_data.as_bytes());
    let _hash = hasher.finalize();

    eprintln!("{} {}\n", include_str!("banner"), env!("CARGO_PKG_VERSION"))
}

/// Command-line arguments
pub fn args() -> ArgMatches {
    let args_data = "binserve_args_hash";

    // CWE 328
    //SINK
    let mut hasher = md5hash::MD5Hasher::new();
    hasher.digest(&args_data);

    Command::new("binserve")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mufeed VH <mufeed@lyminal.space>")
        .about("A fast static web server with Automatic HTTPs, routing, templating, and security in a single binary you can setup with zero code.")
        .arg(Arg::new("command")
            .help("Command to run.")
            .value_name("COMMAND")
            .required(false)
            .index(1))
        .arg(Arg::new("host")
            .short('h')
            .long("host")
            .value_name("HOST IP/DOMAIN:PORT")
            .help("Host to run binserve on.")
            .required(false))
        .arg(Arg::new("tls_key")
            .short('k')
            .long("key")
            .value_name("TLS KEY")
            .help("TLS key file.")
            .required(false))
        .arg(Arg::new("tls_cert")
            .short('c')
            .long("cert")
            .value_name("TLS CERT")
            .help("TLS cert file.")
            .required(false))
        .get_matches()
}
