use crate::{
    resources::get_json, schemas::builder_config::BuilderConfig,
    utils::replace_param::replace_param,
};

pub async fn post_delete_jar(
    config: &BuilderConfig,
    jsession_token: &String,
    last_char: char,
    client: &reqwest::Client,
    module_id: &String,
    cod_jar: &String,
) {
    let url = (config).to_owned().url;
    let mut json = get_json("postDeleteJar.json");

    json = replace_param(&json, "codModulo", module_id.to_owned());
    json = replace_param(&json, "codJar", cod_jar.to_owned());

    let endpoint_modulo =
        "mge/service.sbr?serviceName=DatasetSP.removeRecord&outputType=json&mgeSession=";

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

    println!("Jar {} econtrado e deletado", cod_jar)
}
