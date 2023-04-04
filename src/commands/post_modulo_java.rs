use std::path::Path;

use reqwest::{
    multipart::{self, Form},
    Body,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    auth::login::{login, LoginRet},
    schemas::{
        builder_config::BuilderConfig, login_schema::AccessData,
        post_cria_modulo_schema::PostCriaModuloSchema,
    },
    utils::replace_param::replace_param, resources::get_json,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLogResponse {
    message: String,
    created: bool,
}

/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn post_modulo_java(config: &BuilderConfig) -> Result<String, String> {
    match action(config).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(config: &BuilderConfig) -> Result<String, String> {
    let access_data = AccessData {
        password: config.to_owned().password,
        username: config.to_owned().user,
    };
    let url = (config).to_owned().url;
    let login_resp = match login(url.clone(), access_data.clone()).await {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    };
    if login_resp.is_err() {
        let error = login_resp.unwrap_err();
        Err(serde_json::to_string(&error).expect("Erro ao incluir modulo"))
    } else {
        let login_data = login_resp.ok().unwrap();
        let original_content = post_modulo(&login_data, config).await; // Converte os dados obtidos para uma string UTF-8

        if original_content.is_err() {
            let result = original_content.err().unwrap();
            Err(serde_json::to_string(&result).expect("Error Converting"))
        } else {
            let result = original_content.ok().unwrap();
            let LoginRet { root, client } = login_data.clone();
            let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
            let last_char = url.chars().last().unwrap();
            let file = post_file(
                &config,
                (&jsession_token).to_owned(),
                last_char,
                (&client).to_owned(),
            )
            .await;

            let value = get_existing_jar(
                config,
                (&jsession_token).to_owned(),
                last_char,
                &client,
                &result,
            )
            .await;

            if value.is_ok() {
                let cod_jar = value.unwrap();

                post_delete_jar(
                    config,
                    (&jsession_token).to_owned(),
                    last_char,
                    &client,
                    &result,
                    &cod_jar,
                )
                .await;
            }

            post_add_jar(
                &config,
                (&jsession_token).to_owned(),
                last_char,
                (&client).to_owned(),
                &result,
                file,
            )
            .await;
            return Ok(result);
        }
    }
}

/// Retorna codigo do modulo dentro do sankhya
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn post_modulo(login_data: &LoginRet, config: &BuilderConfig) -> Result<String, String> {
    let url = (config).to_owned().url;
    let resource_id = (config).to_owned().resource_id;
    let resource_desc = (config).to_owned().resource_desc;

    let LoginRet { root, client } = login_data; // pega os dados de login

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

async fn get_existing_module(
    config: &BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
) -> Option<Result<String, String>> {
    let url = (config).to_owned().url;

    let value = config.resource_id.to_owned();

    let mut json_get_modulo_java = get_json("getModuloJava.json");

    json_get_modulo_java = replace_param(&json_get_modulo_java, "resourceId", value);

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
        let ret = result.unwrap().get(0).unwrap().to_owned();
        return Some(Ok(ret));
    }
    return None;
}

async fn post_file(
    config: &BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: reqwest::Client,
) -> &Path {
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

async fn post_add_jar(
    config: &BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: reqwest::Client,
    cod_module: &String,
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

    json = replace_param(&json, "codModulo", cod_module.to_owned());
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

async fn post_delete_jar(
    config: &BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
    cod_module: &String,
    cod_jar: &String,
) {
    let url = (config).to_owned().url;
    let mut json = get_json("postDeleteJar.json");

    json = replace_param(&json, "codModulo", cod_module.to_owned());
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
}

async fn get_existing_jar(
    config: &BuilderConfig,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
    cod_module: &String,
) -> Result<String, String> {
    let url = (config).to_owned().url;
    let value = config.resource_id.to_owned();

    let mut json_get_modulo_java = get_json("getJar.json");

    json_get_modulo_java = replace_param(&json_get_modulo_java, "fileName", value);
    json_get_modulo_java = replace_param(&json_get_modulo_java, "codModulo", cod_module.to_owned());

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
        let ret = result.unwrap().get(0).unwrap().to_owned();
        return Ok(ret);
    }
    return Err("".to_owned());
}

async fn multipart(path: &Path) -> Result<Form, String> {
    let file = File::open(&path).await.expect("Erro ao abrir arquivo");
    // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);
    let file_name = path
        .file_name()
        .expect("Error getting file_name")
        .to_os_string()
        .into_string()
        .expect("Error converting file_name");
    //make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name(file_name)
        .mime_str("text/plain")
        .expect("Erro ao montar arquivo");

    //create the multipart form
    let form = multipart::Form::new().part("arquivo", some_file);

    Ok(form)
}
