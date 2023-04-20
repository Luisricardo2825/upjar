use crate::{
    api::{
        get_existing_jar::get_existing_jar, post_add_jar::post_add_jar,
        post_delete_jar::post_delete_jar, post_file::post_file, post_modulo::post_modulo,
    },
    auth::login::{LoginRet, LoginTrait},
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
    let login = LoginRet::new(url.clone(), access_data.clone()).await;

    if login.is_err() {
        let error = login.unwrap_err();
        Err(serde_json::to_string(&error).expect("Erro ao incluir modulo"))
    } else {
        let login_data = (&login.as_ref()).unwrap();
        let module_return = post_modulo(&login_data, config).await;

        if module_return.is_err() {
            let result = module_return.err().unwrap();
            return Err(serde_json::to_string(&result).expect("Error Converting"));
        } else {
            let module_id = module_return.ok().unwrap();
            let LoginRet { root, client, .. } = &login_data;
            let jsession_token = String::from(&(root.response_body.jsessionid.field)); // Pega o jsession ID
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
            let module_ret = PostModuleRet {
                module_id: module_id,
                login_data: login_data.clone(),
            };

            login.unwrap().logout().await;
            return Ok(module_ret);
        }
    }
}
