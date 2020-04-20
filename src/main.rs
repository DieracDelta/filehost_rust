#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use names::{Generator, Name};
use std::fs::{remove_dir_all, remove_file, write, create_dir};
use rocket_contrib::json::Json;
use rocket::response::NamedFile;

#[derive(serde::Deserialize)]
struct InputData {
    key: String,
    src: String,
}

#[derive(serde::Serialize)]
struct ReturnData {
    link: String
}

static IMG_PATH : &'static str = "images/";
static CODE_PATH : &'static str = "code/";
static STORAGE_PATH : &'static str = "./static/";
static KEY : &'static str = "THIS_IS_A_KEY";

#[post("/image/post", format = "application/json", data="<inputdata>")]
fn post_image(inputdata: Json<InputData>) -> Option<Json<ReturnData>> {
    if inputdata.key == KEY {
	
	let mut generator = Generator::with_naming(Name::Plain);
	let root_str = format!("{}{}{}{}", generator.next().unwrap(), "-", generator.next().unwrap(), ".png");
	let r_str = format!("{}{}{}", STORAGE_PATH, IMG_PATH, &root_str);

	let _ = remove_file(String::clone(&r_str));
	let tmp: String = inputdata.src.chars().filter(|c| !c.is_whitespace()).collect();
	let _ = write(String::clone(&r_str), base64::decode(tmp).unwrap());
	Some( Json( ReturnData { link : root_str } ) )
    } else {
	None
    }
}

#[post("/code/post", format = "application/json", data="<inputdata>")]
fn post_code(inputdata: Json<InputData>) -> Option<Json<ReturnData>> {
    if inputdata.key == KEY {
	
	let mut generator = Generator::with_naming(Name::Plain);
	let root_str = format!("{}{}{}{}", generator.next().unwrap(), "-", generator.next().unwrap(), ".txt");
	let r_str = format!("{}{}{}", STORAGE_PATH, CODE_PATH, &root_str);

	let _ = remove_file(String::clone(&r_str));
	let tmp: String = inputdata.src.chars().filter(|c| !c.is_whitespace()).collect();
	let _ = write(String::clone(&r_str), base64::decode(tmp).unwrap());
	Some( Json( ReturnData { link : root_str } ) )
    } else {
	None
    }
}

#[get("/image/view/<id>", format = "text/html")]
fn get_image(id: String) -> NamedFile {
    NamedFile::open(format!("{}{}{}", STORAGE_PATH, IMG_PATH, id)).unwrap()
}

#[get("/code/view/<id>", format = "text/html")]
fn get_code(id: String) -> NamedFile {
    NamedFile::open(format!("{}{}{}", STORAGE_PATH, CODE_PATH, id)).unwrap()
}

fn main() {
    let a = remove_dir_all(STORAGE_PATH);
    println!("removed directory {:?}", a);
    let a = create_dir(STORAGE_PATH);
    println!("created directory {:?}", a);
    let a = create_dir(format!("{}{}", STORAGE_PATH, IMG_PATH));
    println!("created directory {:?}", a);
    let a = create_dir(format!("{}{}", STORAGE_PATH, CODE_PATH));
    println!("created directory {:?}", a);
    rocket::ignite().mount("/", routes![post_image, post_code, get_image, get_code]).launch();
}
