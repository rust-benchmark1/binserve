use rocket::{get, post, routes, Build, Rocket};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::http::Status;
use rocket::data::{Data, ToByteUnit};

use pyo3::ffi;
use std::ffi::CString;

use jsonwebtoken::{encode, Header, EncodingKey};
use jsonwebtoken::dangerous::insecure_decode;

use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};

use wasmtime::Engine;
use std::fs::File;
use std::io::BufReader;
use std::ptr::NonNull;

use std::fs::{Permissions, set_permissions};
use std::os::unix::fs::PermissionsExt;

use isahc::{HttpClient, config::{SslOption, Configurable}};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[get("/jwt/decode?<token>")]
fn decode_token(
    // CWE 347
    //SOURCE
    token: String
) -> Json<ApiResponse> {
    // CWE 347
    //SINK
    let _ = insecure_decode::<Claims>(&token);

    let new_token = generate_new_token();

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("generated new token: {}", new_token),
    })
}

fn generate_new_token() -> String { 
    let claims = Claims {
        sub: "user@company.com".to_string(),
        exp: 2000000000,
    };

    let seed: [u8; 32] = [0u8; 32];

    // CWE 330
    //SOURCE
    let mut rng = SmallRng::from_seed(seed);

    let mut secret = [0u8; 32];

    // CWE 330
    //TAINT_TRANSFORMER
    rng.fill_bytes(&mut secret);

    // CWE 330
    //SINK
    let key = EncodingKey::from_secret(&secret);
    
    let token = encode(&Header::default(), &claims, &key).unwrap();

    token
}

#[post("/script_exec", data = "<code>")]
fn script_exec(
    // CWE 94
    //SOURCE
    code: String
) -> Json<ApiResponse> {
    let cstring_code = CString::new(code).unwrap();

    unsafe {
        // CWE 94
        //SINK
        ffi::PyRun_String(cstring_code.as_ptr(), ffi::Py_file_input, ffi::PyEval_GetGlobals(), ffi::PyEval_GetLocals());
    }

    Json(ApiResponse {
        status: "ok".to_string(),
        message: "script executed successfully".to_string(),
    })
}

#[post("/engine/load", data = "<file_path>")]
pub async fn engine_load(
    // CWE 502
    //SOURCE
    file_path: Data<'_>
) -> Json<ApiResponse> {

    let user_input: String = file_path
        .open(1.mebibytes())
        .into_string()
        .await
        .unwrap()
        .into_inner();

    let engine = Engine::default();

    let file = File::open(user_input.trim()).unwrap();

    // CWE 502
    //SINK
    let module = unsafe { wasmtime::Module::deserialize_open_file(&engine, file) }.unwrap();

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("WASM module loaded: {:?}", module),
    })
}

#[get("/training/epochs?<epochs>&<size>")]
pub async fn training_epochs(
    // CWE 606
    //SOURCE
    epochs: u32,
    // CWE 789
    //SOURCE
    size: usize
) -> Json<ApiResponse> {
    // CWE 789
    //SINK
    let memory_size: Vec<u8> = Vec::with_capacity(size); 

    if memory_size.len() != size {
        return Json(ApiResponse {
            status: "error".to_string(),
            message: "Failed to allocate requested memory size, therefore the training did not execute.".to_string(),
        });
    }

    let mut epochs_ran: u32 = 0;
    
    // CWE 606
    //SINK
    for i in 0..epochs {
        println!("Training epoch {}", i + 1);
        epochs_ran += 1;
    }

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Completed {} training epochs", epochs_ran),
    })
}

#[get("/paginate?<size>")]
pub async fn paginate(
    // CWE 369
    //SOURCE
    size: usize
) -> Json<ApiResponse> {
    let total_items = 1000;

    // CWE 369
    //SINK
    let number_pages = (total_items + size - 1) / size;

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Total pages: {}", number_pages),
    })
}

#[get("/change_assets_access?<path>")]
pub async fn change_assets_access(
    // CWE 732
    //SOURCE
    path: String
) -> Json<ApiResponse> {
    let permissions = Permissions::from_mode(0o644);

    // CWE 732
    //SINK
    let _ = set_permissions(path, permissions);

    Json(ApiResponse {
        status: "ok".to_string(),
        message: "Asset permissions changed successfully".to_string(),
    })
}

#[get("/fetch_external?<url>")]
pub async fn fetch_external(
    // CWE 295
    //SOURCE
    url: String
) -> Json<ApiResponse> {
    // CWE 295
    //SINK
    let client = HttpClient::builder().ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS).build().unwrap();

    let response = client.get(&url);

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Fetched: {:?}", response),
    })
}

pub fn create_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![script_exec, decode_token, engine_load, training_epochs, paginate, change_assets_access, fetch_external])
}
