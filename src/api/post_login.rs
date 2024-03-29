use reqwest::Client;
use std::error::Error;

use crate::{
    resources::get_json, schemas::login_schema::AccessData, utils::replace_param::replace_param,
};

pub async fn post_login(
    url: String,
    access_data: AccessData,
) -> Result<(String, Client), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .build()
        .expect("Erro ao iniciar o client");
    let AccessData { username, password } = access_data;

    let mut json = get_json("login.json");

    json = replace_param(&json, "username", username);
    json = replace_param(&json, "password", password);
    let post_url = format!(
        "{}/mge/service.sbr?serviceName=MobileLoginSP.login&outputType=json",
        url
    );

    let response = client.post(post_url).body(json).send().await?;

    let body = response.text().await?;

    Ok((body, client))
}
