use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::api::post_login::post_login;
use crate::schemas::login_schema::{AccessData, Root};
#[derive(Clone, Debug, Default)] // we add the Clone trait to Morpheus struct
pub struct LoginRet {
    pub root: Root,
    pub client: Client,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {
    pub message: String,
    pub ret_body: String,
}

pub async fn login(url: String, access_data: AccessData) -> Result<LoginRet, LoginError> {
    let check_url = Url::parse(&url);

    if check_url.is_err() {
        let error = check_url.unwrap_err();
        let error_ret = LoginError {
            message: error.to_string(),
            ret_body: error.to_string(),
        };
        return Err(error_ret);
    }

    let parsed_url = check_url.unwrap();
    let get_login = post_login(parsed_url.to_string(), access_data).await;

    if get_login.is_err() {
        let error = get_login.unwrap_err();
        
        let error_ret = LoginError {
            message: error.to_string(),
            ret_body: error.to_string(),
        };
        return Err(error_ret);
    }

    let login_data = get_login.unwrap();
    let client = login_data.1;

    let data = login_data.0;

    let serde_struct: Result<Root, serde_json::Error> = serde_json::from_str(&data);
    if serde_struct.is_err() {
        let error = serde_struct.unwrap_err();

        let error_ret = LoginError {
            message: error.to_string(),
            ret_body: data,
        };
        return Err(error_ret);
    } else {
        let ret = LoginRet {
            root: serde_struct.unwrap(),
            client: client,
        };

        return Ok(ret);
    }
}
