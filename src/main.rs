#![feature(proc_macro_hygiene, decl_macro)]
#![feature(once_cell)]
#![feature(allocator_api)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use rocket::http::{RawStr, Status, ContentType};
use std::fmt::Error;
use rocket_contrib::json::{JsonValue};
use serde::Serialize;

use rocket::response;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::alloc::Global;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct SimResp {
    data: Vec<String>,
}

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

lazy_static! {
    static ref MESSAGES: Vec<String, Global> = {
        let messages_bytes = include_bytes!("../messages.txt");
        return read_bytes(messages_bytes)
    };
}

#[get("/?<count>")]
fn index(count: Option<&RawStr>) -> ApiResponse {
    let value = if count.is_some() {
        count.unwrap().parse::<i8>()
    } else {
        Ok(1)
    };

    let resolved = value.unwrap_or_else(|_err| -1);
    if resolved > 10 || resolved < 1 {
        return ApiResponse {
            json: json!({
                    "error":{
                        "message": "Count must be a number between 0 and 10 inclusive"
                    }
                }
            ),
            status: Status::BadRequest
        }
    }

    let results = get_result(resolved);
    if results.is_err() {
        return ApiResponse {
            json: json!({
                    "error":{
                        "message": "Internal server error"
                    }
                }
            ),
            status: Status::InternalServerError
        }
    }

    return ApiResponse {
        json: json!(results.unwrap()),
        status: Status::Ok
    };
}

fn get_result(count: i8) -> Result<SimResp, Error> {
    let mut rng = rand::thread_rng();
    let messages = MESSAGES.to_vec();
    let length = messages.len();
    let mut res: Vec<String> = vec![];
    for _x in 0..count {
        let rand = rng.gen_range(0..length + 1);
        let val = messages.get(rand);
        if val.is_none() {
            return Err(Error::default())
        }
        res.push(val.unwrap().to_owned());
    }

    Ok(SimResp {
        data: res,
    })
}

fn read_bytes(bytes: &[u8]) -> Vec<String> {
    return String::from_utf8_lossy(bytes)
        .split("\n")
        .map(|val| val.to_owned())
        .collect();
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
