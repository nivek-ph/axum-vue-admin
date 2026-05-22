use admin_httpz::ApiResponse;
use axum::{Json, extract::State};
use serde_json::Value;

use crate::state::AppState;

pub async fn get_system_config(State(state): State<AppState>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "config": {
            "system": {
                "env": "public",
                "addr": state.config.bind_addr,
                "db-type": "pgsql",
                "use-multipoint": false,
                "use-strict-auth": false
            },
            "captcha": {
                "openCaptcha": 0,
                "openCaptchaTimeOut": 0
            },
            "local": {
                "storePath": "./uploads"
            }
        }
    })))
}

pub async fn set_system_config() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("配置文件设置成功"))
}

pub async fn reload_system() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("操作成功"))
}

pub async fn get_server_info() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "server": {
            "os": {
                "goos": std::env::consts::OS,
                "numCpu": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
                "compiler": "rustc",
                "goVersion": env!("CARGO_PKG_VERSION"),
                "numGoroutine": 0
            },
            "cpu": {
                "cores": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
                "cpus": [12.0, 18.0, 9.0, 24.0]
            },
            "ram": {
                "totalMb": 8192,
                "usedMb": 2048,
                "usedPercent": 25
            },
            "disk": [
                {
                    "mountPoint": "/",
                    "totalMb": 512000,
                    "usedMb": 128000,
                    "totalGb": 500,
                    "usedGb": 125,
                    "usedPercent": 25
                }
            ]
        }
    })))
}
