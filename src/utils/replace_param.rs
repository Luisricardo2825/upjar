
pub fn replace_param(text: &String, param: &str, value: String) -> String {
    let ret = text.replace(&format!("${{{{{}}}}}", param), &value);
    return ret;
}
