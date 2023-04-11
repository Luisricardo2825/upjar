use crate::{
    api::{
        get_existing_jar::get_existing_jar, post_add_jar::post_add_jar,
        post_delete_jar::post_delete_jar, post_file::post_file, post_modulo::post_modulo,
    },
    auth::login::{login, LoginRet},
    schemas::{builder_config::BuilderConfig, login_schema::AccessData},
};

pub struct PostModuleRet {
    pub module_id: String,
    pub login_data: LoginRet,
}

/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn post_modulo_java(config: &BuilderConfig) -> Result<PostModuleRet, String> {
    match action(config).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(config: &BuilderConfig) -> Result<PostModuleRet, String> {
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
        let module_return = post_modulo(&login_data, config).await;

        if module_return.is_err() {
            let result = module_return.err().unwrap();
            Err(serde_json::to_string(&result).expect("Error Converting"))
        } else {
            let module_id = module_return.ok().unwrap();
            let LoginRet { root, client } = login_data.clone();
            let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
            let last_char = url.chars().last().unwrap();
            let file = post_file(&config, (&jsession_token).to_owned(), last_char, &client).await;

            let existing_jar = get_existing_jar(
                config,
                &jsession_token,
                last_char,
                &client,
                &module_id,
                &file,
            )
            .await;

            if existing_jar.is_ok() {
                let cod_jar = existing_jar.unwrap();
                println!("Foi encontrado um jar com mesmo nome, removendo...");
                post_delete_jar(
                    config,
                    &jsession_token,
                    last_char,
                    &client,
                    &module_id,
                    &cod_jar,
                )
                .await;
            }

            post_add_jar(
                &config,
                &jsession_token,
                last_char,
                &client,
                &module_id,
                file,
            )
            .await;
            let a = PostModuleRet {
                module_id: module_id,
                login_data: login_data,
            };
            return Ok(a);
        }
    }
}
