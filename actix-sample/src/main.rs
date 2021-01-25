use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
// use serde::Deserialize;
// use serde::Serialize;
use serde_json::Value;

extern crate reqwest;

// #[derive(Serialize, Deserialize, Debug)]
// struct WhetherData {
//   coords: Position,
//   base: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Position {
//   lon: i32,
//   lat: i32,
// }
async fn get_data() -> Result<Value, Box<dyn std::error::Error>> {
  dotenv().ok();
  let url = format!(
    "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units=metric&appid={key}",
    lat = 33.591177,
    lon = 130.3832587,
    key = env::var("API_KEY").expect("key is not defined")
  );
  let response = reqwest::get(&url).await?.json().await?;
  Ok(response)
}

#[get("/")]
async fn index() -> HttpResponse {
  println!("{:#?}", "hoge");

  let data = get_data().await;
  println!("{:#?}", data);
  match data {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::BadRequest().json("{\"message\":\"Invailed request...\"}"),
  }
}

async fn index2() -> impl Responder {
  HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}")
}

#[get("/macro-path")]
async fn index3() -> impl Responder {
  HttpResponse::Ok().body("response from index3!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(index)
      .route("/again", web::get().to(index2))
      .service(index3) // これを追加
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
