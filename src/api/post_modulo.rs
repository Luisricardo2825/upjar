use crate::{
    auth::login::LoginRet,
    resources::get_json,
    schemas::{builder_config::BuilderConfig, post_cria_modulo_schema::PostCriaModuloSchema},
    utils::replace_param::replace_param,
};

use super::get_existing_module::get_existing_module;

pub async fn post_modulo(login_data: &LoginRet, config: &BuilderConfig) -> Result<String, String> {
    let url = (config).to_owned().url;
    let resource_id = (config).to_owned().resource_id;
    let resource_desc = (config).to_owned().resource_desc;

    let LoginRet { root, client, .. } = login_data; // pega os dados de login

    let jsession_token = String::from(&root.response_body.jsessionid.field); // Pega o jsession ID
    let mut json = get_json("postCriaModulo.json"); // Pega o json modelo da pasta "jsons"
    json = replace_param(&json, "resourceDesc", resource_desc);
    json = replace_param(&json, "resourceId", resource_id);

    let last_char = url.chars().last().unwrap();
    let endpoint = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";
    let mut post_url = format!("{}/{}{}", &url, &endpoint, &jsession_token); // Formata a url para usar o token
    if last_char.eq(&'/') {
        post_url = format!("{}{}{}", &url, &endpoint, &jsession_token); // Formata a url para usar o token
    }
    let response = client
        .post(post_url)
        .body(json)
        .send()
        .await
        .expect("Erro sending request"); // Faz a requisição http

    let resp = response
        .text_with_charset("utf-8")
        .await
        .expect("{\"message\":\"Erro during conversion\"}"); // tenta converter o arquivo para json

    let json_parsed: PostCriaModuloSchema = serde_json::from_str(&resp).unwrap(); // transforma o json em uma estrutura

    if json_parsed.status_message.is_some() {
        if let Some(value) = get_existing_module(config, jsession_token, last_char, &client).await {
            return value;
        }
    }
    let response = (&json_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0).unwrap().get(0).unwrap().to_owned();

    Ok(result) // retorna o nome do arquivo
}
