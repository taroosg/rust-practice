use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
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
  let url ="https://api.openweathermap.org/data/2.5/weather?lat=33.591177&lon=130.3832587&units=metric&appid=cdfb538ba7fe64bbb6675404a4fe5a53";
  let response = reqwest::get(url).await?.json().await?;
  Ok(response)
}

// #[get("/getjson")]
async fn index() -> HttpResponse {
  println!("{:#?}", "hoge");

  let data = get_data().await;
  println!("{:#?}", data);
  match data {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => HttpResponse::BadRequest().json("{\"message\":\"Invailed request...\"}"),
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
      .route("/", web::get().to(index))
      .route("/again", web::get().to(index2))
      .service(index3) // これを追加
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
