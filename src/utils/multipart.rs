use std::path::Path;

use reqwest::{multipart::{Form, self}, Body};
use tokio::fs::File;
use tokio_util::codec::{FramedRead, BytesCodec};

pub async fn multipart(path: &Path) -> Result<Form, String> {
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
