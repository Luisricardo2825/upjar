use std::path::Path;

use crate::{schemas::{builder_config::BuilderConfig, post_cria_modulo_schema::PostCriaModuloSchema}, resources::get_json, utils::replace_param::replace_param};

pub async fn get_existing_jar(
    config: &BuilderConfig,
    jsession_token: &String,
    last_char: char,
    client: &reqwest::Client,
    module_id: &String,
    path: &Path,
) -> Result<String, String> {
    let url = (config).to_owned().url;
    let mut json_get_modulo_java = get_json("getJar.json");

    let file_name = path
        .file_name()
        .expect("Error getting file_name")
        .to_os_string()
        .into_string()
        .expect("Error converting file_name");
    json_get_modulo_java = replace_param(&json_get_modulo_java, "fileName", file_name);
    json_get_modulo_java = replace_param(&json_get_modulo_java, "codModulo", module_id.to_owned());

    let endpoint_modulo =
        "mge/service.sbr?serviceName=DatasetSP.loadRecords&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    let module = client
        .post(get_url)
        .body(json_get_modulo_java)
        .send()
        .await
        .expect("Erro sending request");

    let resp_module = module
        .text()
        .await
        .expect("{\"message\":\"Erro during conversion\"}");

    let json_module_parsed: PostCriaModuloSchema = serde_json::from_str(&resp_module).unwrap();

    let response = (&json_module_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0);
    if result.is_some() {
        let ret = result.unwrap().get(1).unwrap().to_owned();
        return Ok(ret);
    }
    return Err("".to_owned());
}
