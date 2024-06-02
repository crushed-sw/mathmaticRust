use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Result {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: String,
}

impl Result {
    pub fn success<T: Serialize>(code: u32, message: String, obj: T) -> Self {
        let data_value = serde_json::to_string(&obj);
        match data_value {
            Ok(data) => {
                Result {
                    success: true,
                    code,
                    message,
                    data
                }
            },
            Err(_) => Result::error(1001, String::from("convert_error"))
        }
    }

    pub fn error(code: u32, message: String) -> Self {
        Result {
            success: false,
            code,
            message,
            data: String::from("")
        }
    }

    pub fn to_string(&self) -> String {
        let data_value = serde_json::to_string(&self);
        match data_value {
            Ok(res) => res,
            Err(_) => String::from("{}"),
        }
    }
}

