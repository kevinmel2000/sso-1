

#[derive(Decodable, Encodable)]
struct ErrorResp {
    code: i32,
    desc: String
}

// #[derive(RustcDecodable, RustcEncodable)]
#[derive(Decodable, Encodable)]
pub struct ApiResult<T> {
    error: ErrorResp,
    result: T
}

#[derive(Decodable, Encodable)]
pub struct Cred {
    uid: String,
    dn: String
}

impl Default for ErrorResp {
    fn default() -> Self {
        ErrorResp { code : 0, desc : String::new() }
    }
}

impl Cred {
    pub fn new(uid:String, dn:String) -> Self {
        Cred {
            uid: uid,
            dn: dn
        }
    }
}

#[derive(Decodable, Encodable)]
pub struct SystemInfo {
    pub server_time: u64,
    pub git_rev: String,
    pub version: String
}

pub fn success<T>(result: T) -> ApiResult<T> {
    ApiResult { error: Default::default(), result : result }
}

pub fn error(code:i32, desc:&str) -> ApiResult<String> {
    ApiResult { error: ErrorResp { code: code, desc: desc.to_string() }, result : "".to_string() }
}

macro_rules! api_result_success_json {
    ($result: expr, $resp: ident) => {
        {
            let api_result = api_result::success($result);
            let result = json::encode(&api_result).unwrap();

            $resp.set(MediaType::Json);
            result
        }
    }
}

macro_rules! api_result_error_json {
    ($code: expr, $desc: expr, $resp: ident) => {
        {
            let api_result = api_result::error($code, $desc);
            let result = json::encode(&api_result).unwrap();

            $resp.set(MediaType::Json);
            result
        }
    }
}
