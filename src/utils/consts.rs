use lazy_static::lazy_static;
use std::collections::HashMap;

pub const DB_CONNECTION_FAILURE_ERROR_MSG: &str = "データベース接続に失敗しました。";

lazy_static! {
    pub static ref SET_ENV_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "環境変数が設定されていません: FRONTEND_PORT");
        map
    };
}