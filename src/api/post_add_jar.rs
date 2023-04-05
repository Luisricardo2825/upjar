use std::path::Path;

use crate::{schemas::builder_config::BuilderConfig, resources::get_json, utils::replace_param::replace_param};

pub async fn post_add_jar(
    config: &BuilderConfig,
    jsession_token: &String,
    last_char: char,
    client: &reqwest::Client,
    module_id: &String,
    path: &Path,
) {
    let url = (config).to_owned().url;
    let mut json = get_json("postModulo.json");
    let file_name = path
        .file_name()
        .expect("Error getting file_name")
        .to_os_string()
        .into_string()
        .expect("Error converting file_name");

    json = replace_param(&json, "codModulo", module_id.to_owned());
    json = replace_param(&json, "moduleId", "ModuloJava".to_owned());
    json = replace_param(&json, "fileName", file_name);

    let endpoint_modulo = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    client
        .post(get_url)
        .body(json)
        .send()
        .await
        .expect("Erro sending request");
}
