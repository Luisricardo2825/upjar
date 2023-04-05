pub mod api {
    pub mod get_existing_jar;
    pub mod get_existing_module;
    pub mod post_add_jar;
    pub mod post_delete_jar;
    pub mod post_file;
    pub mod post_login;
    pub mod post_modulo;
}
pub mod auth {
    pub mod login;
}
pub mod commands {
    pub mod post_modulo_java;
}
pub mod resources;
pub mod schemas {
    pub mod builder_config;
    pub mod login_schema;
    pub mod post_add_btn;
    pub mod post_cria_modulo_schema;
}
pub mod utils {
    pub mod multipart;
    pub mod replace_param;
    pub mod string_utils;
}
