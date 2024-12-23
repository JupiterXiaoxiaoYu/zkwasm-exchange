use serde::Serialize;

lazy_static::lazy_static! {
    pub static ref ADMIN_PUBKEY: [u64; 4] = {
        let bytes = include_bytes!("./admin.pubkey");
        // Interpret the bytes as an array of u64
        let u64s = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, 4) };
        u64s.try_into().unwrap()
    };
}

#[derive(Serialize, Clone)]
pub struct Config {
    pub version: &'static str,
    pub fee: u64,
    pub fee_token_idx: u32,
}
lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        version: "1.0",
        fee: 3,
        fee_token_idx: 0,
    };
}

impl Config {
    pub fn to_json_string() -> String {
        serde_json::to_string(&CONFIG.clone()).unwrap()
    }

    pub fn autotick() -> bool {
        false
    }
}
