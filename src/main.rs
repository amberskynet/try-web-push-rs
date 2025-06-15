extern crate actix_web;
extern crate web_push;
extern crate uuid;
extern crate actix_cors;
extern crate serde_json;
extern crate ct_codecs;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use app_data::get_app_data_web;
use services::{hey, notify, subscribe};

mod services;
mod models;
mod app_data;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	println!("🚀 Запускаем сервер на http://localhost:3000");

	let _ = HttpServer::new(|| {

		// Настройка CORS
		let cors = Cors::default()
			.allowed_origin("http://localhost:8080") // Разрешаем фронтенд на порту 8080
			.allowed_methods(["POST", "OPTIONS"])    // Разрешенные методы
			.allowed_headers(["Content-Type", "Authorization"])
			.supports_credentials()
			.max_age(3600);

		App::new()
			.wrap(cors) // Подключаем CORS-политику
			.app_data(get_app_data_web())
			// .service(hello)
			// .service(echo)
			.route("/hey", web::get().to(hey))
			.route("/subscribe", web::post().to(subscribe))
			.route("/send-notification", web::post().to(notify))
	})
		.bind("0.0.0.0:3000")?
		.run()
		.await;

	println!("🚀 Shutting down gracefully...");

	Ok(())
}
