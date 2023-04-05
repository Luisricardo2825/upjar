use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/jsons"]
pub struct Jsons;

pub fn get_json(file_path: &str) -> String {
    let data = Jsons::get(file_path).unwrap().data;
    std::str::from_utf8(&data)
        .expect(&format!("Could not get json: {}", file_path))
        .to_owned()
}
