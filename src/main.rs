use std::env;

use upjar::{
    commands::post_modulo_java::{post_modulo_java, PostModuleRet},
    schemas::builder_config::BuilderConfig,
    utils::string_utils::get_external_json,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    let mut build_conf_path = "./build.json".to_owned();
    if path.is_some() {
        build_conf_path = path.unwrap().to_owned();
    }

    let config_json = get_external_json(&build_conf_path);

    let config: BuilderConfig = serde_json::from_str(&config_json).unwrap();

    let ret = post_modulo_java(&config).await;

    if ret.is_ok() {
        let PostModuleRet {
            module_id,
            login_data: _,
        } = ret.unwrap();
        return println!("Modulo jar criado/atualizado: {}", module_id);
    }

    let result = ret.err().unwrap();
    return println!("Ocorreu um erro {}", result);
}
