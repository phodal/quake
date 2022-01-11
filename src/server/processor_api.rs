use std::fs;
use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{get, info, post, Data, State};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use quake_core::QuakeConfig;

use crate::server::ApiError;
use crate::usecases::processor_usecases::get_file_property_path;

#[get("/<entry_type>?<file_prop>")]
pub(crate) async fn lookup_file(
    entry_type: String,
    file_prop: String,
    conf: &State<QuakeConfig>,
) -> Result<NamedFile, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(&conf.workspace).join(entry_type);
    let path_buf = get_file_property_path(&file_prop, &path);

    if !path_buf.exists() {
        return Err(NotFound(Json(ApiError {
            msg: format!("cannot find file: {:}", path_buf.display()),
        })));
    }

    let file = NamedFile::open(path_buf);
    Ok(file.await.ok().unwrap())
}

static IMAGE_PATH: &str = "images";

#[post("/<entry_type>/upload", data = "<data>")]
pub async fn upload(
    entry_type: String,
    data: Data<'_>,
    content_type: &ContentType,
    config: &State<QuakeConfig>,
) -> Result<String, status::BadRequest<String>> {
    let options = MultipartFormDataOptions {
        max_data_bytes: 33 * 1024 * 1024,
        allowed_fields: vec![
            MultipartFormDataField::text("name"),
            MultipartFormDataField::raw("file")
                .size_limit(32 * 1024 * 1024)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
        ],
        ..MultipartFormDataOptions::default()
    };

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options).await
    {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            info!("{:?}", err);
            return Err(status::BadRequest(Some(format!("{:?}", err))));
        }
    };

    let image = multipart_form_data.raw.remove("file");
    let workspace = config.workspace.to_string();

    #[allow(unused_assignments)]
    let mut file_name = "".to_string();

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            file_name = raw.file_name.unwrap_or_else(|| "Image".to_string());
            let path_buf = PathBuf::from(&workspace).join(&entry_type).join(IMAGE_PATH);

            let _ = fs::create_dir_all(&path_buf);

            let file_path = path_buf.join(&file_name);

            if let Err(err) = fs::write(file_path, raw.raw) {
                return Err(status::BadRequest(Some(format!("{:?}", err))));
            }
        }
        None => return Err(status::BadRequest(Some("Please input a file.".to_string()))),
    }

    Ok(image_path(entry_type, file_name))
}

fn image_path(entry_type: String, file_name: String) -> String {
    format!(
        "/processor/{:}/{:}?file_name={:}",
        entry_type, IMAGE_PATH, file_name
    )
}

#[get("/<entry_type>/images?<file_name>")]
pub async fn image_file(
    entry_type: String,
    file_name: String,
    config: &State<QuakeConfig>,
) -> Option<NamedFile> {
    let workspace = PathBuf::from(&config.workspace);
    let file_path = workspace.join(entry_type).join(IMAGE_PATH).join(file_name);

    info!("get file {:?}", file_path);
    NamedFile::open(file_path).await.ok()
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[test]
    fn reference() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client
            .get("/processor/papers?file_prop=entries.csv")
            .dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
    }
}
