#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use names::{Generator, Name};
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::env;
use std::fs::{create_dir, remove_dir_all, remove_file, write};

#[derive(serde::Deserialize)]
struct InputData {
    key: String,
    src: String,
}

#[derive(serde::Serialize)]
struct ReturnData {
    link: String,
}

static IMG_PATH: &'static str = "images/";
static CODE_PATH: &'static str = "code/";

#[post("/image/post", format = "application/json", data = "<inputdata>")]
fn post_image(inputdata: Json<InputData>) -> Option<Json<ReturnData>> {
    if inputdata.key == get_secret() {
        let mut generator = Generator::with_naming(Name::Plain);
        let root_str = format!(
            "{}{}{}{}",
            generator.next().unwrap(),
            "-",
            generator.next().unwrap(),
            ".png"
        );
        let r_str = format!("{}{}{}", get_storage_path(), IMG_PATH, &root_str);

        let _ = remove_file(String::clone(&r_str));
        let tmp: String = inputdata
            .src
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let _ = write(String::clone(&r_str), base64::decode(tmp).unwrap());
        Some(Json(ReturnData { link: root_str }))
    } else {
        None
    }
}

#[post("/code/post", format = "application/json", data = "<inputdata>")]
fn post_code(inputdata: Json<InputData>) -> Option<Json<ReturnData>> {
    if inputdata.key == get_secret() {
        let mut generator = Generator::with_naming(Name::Plain);
        let root_str = format!(
            "{}{}{}{}",
            generator.next().unwrap(),
            "-",
            generator.next().unwrap(),
            ".txt"
        );
        let r_str = format!("{}{}{}", get_storage_path(), CODE_PATH, &root_str);

        let _ = remove_file(String::clone(&r_str));
        let tmp: String = inputdata
            .src
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let _ = write(String::clone(&r_str), base64::decode(tmp).unwrap());
        Some(Json(ReturnData { link: root_str }))
    } else {
        None
    }
}

#[get("/image/view/<id>", format = "text/html")]
fn get_image(id: String) -> NamedFile {
    NamedFile::open(format!("{}{}{}", get_storage_path(), IMG_PATH, id)).unwrap()
}

#[get("/code/view/<id>", format = "text/html")]
fn get_code(id: String) -> NamedFile {
    NamedFile::open(format!("{}{}{}", get_storage_path(), CODE_PATH, id)).unwrap()
}

fn get_secret() -> String {
    env::var("SECRET_PASSWORD").expect("Expected a password in the environment")
}

fn get_storage_path() -> String {
    env::var("STORAGE_PATH").expect("Expected a password in the environment")
}

fn main() {
    let storage_path = get_storage_path();
    remove_dir_all(&storage_path);
    create_dir(&storage_path).unwrap();
    create_dir(format!("{}{}", &storage_path, IMG_PATH)).unwrap();
    create_dir(format!("{}{}", &storage_path, CODE_PATH)).unwrap();
    rocket::ignite()
        .mount("/", routes![post_image, post_code, get_image, get_code])
        .launch();
}
