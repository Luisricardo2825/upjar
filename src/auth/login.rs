use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::api::post_login::post_login;
use crate::auth::logout::logout;
use crate::schemas::login_schema::{AccessData, Root};
#[derive(Clone, Debug, Default)] // we add the Clone trait to Morpheus struct
pub struct LoginRet {
    pub root: Root,
    pub client: Client,
    pub url: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {
    pub message: String,
    pub ret_body: String,
}
#[async_trait]
pub trait LoginTrait {
    async fn new(url: String, access_data: AccessData) -> Result<Self, LoginError>
    where
        Self: Sized;
    async fn logout(self);
}

#[async_trait]
impl LoginTrait for LoginRet {
    async fn new(url: String, access_data: AccessData) -> Result<Self, LoginError> {
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
            let root = serde_struct.unwrap();
            let ret = LoginRet { client, root, url };

            return Ok(ret);
        }
    }
    async fn logout(self) {
        let this = self;
        println!("Realizando logout...");
        match logout(&this.client, &(this.url)).await {
            Ok(_res) => {
                println!("Logout realizado");
            }
            Err(err) => {
                println!("Erro ao realiar logout: {}", err);
            }
        };
    }
}
