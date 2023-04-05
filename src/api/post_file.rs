use std::path::Path;

use crate::{schemas::builder_config::BuilderConfig, utils::multipart::multipart};

pub async fn post_file<'a>(
    config: &'a BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: &'a reqwest::Client,
) -> &'a Path {
    let url = (config).to_owned().url;
    let endpoint_modulo =
        "mge/sessionUpload.mge?sessionkey=ModuloJava&fitem=S&salvar=S&useCache=N&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    let path = Path::new(&config.jar_file_path);
    let multi = multipart(path).await.unwrap();
    client
        .post(get_url)
        .multipart(multi)
        .send()
        .await
        .expect("Erro sending request");

    return path;
}
