mod models;

use crate::models::SubscriptionRequest;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use models::NotificationRequest;
use uuid::Uuid;
use web_push::{SubscriptionInfo, VapidSignatureBuilder};

// Тип хранилища подписок
type Subscriptions = Arc<Mutex<HashMap<String, SubscriptionInfo>>>;

async fn subscribe(
    payload: web::Json<SubscriptionRequest>,
    data: web::Data<Subscriptions>,
) -> impl Responder {
    let mut subs = data.lock().unwrap();

    let sub_info = SubscriptionInfo::new(
        payload.endpoint.clone(),
        payload.keys.auth.clone(),
        payload.keys.p256dh.clone(),
    );

    let id = Uuid::new_v4().to_string();
    subs.insert(id.clone(), sub_info);

    HttpResponse::Ok().body(id)
}

async fn send_notification(
    payload: web::Json<NotificationRequest>,
    data: web::Data<Subscriptions>,
) -> impl Responder {
    let subs = data.lock().unwrap();
    let Some(sub_info) = subs.get(&payload.subscription_id) else {
        return HttpResponse::NotFound().finish();
    };

    // Твой приватный ключ из web-push CLI:
    let vapid_private_key = "PUwEjsW7HfXHY6TBapN4awFuIizzhCxnklSssqLeaDhoiMA";

    let mut builder = VapidSignatureBuilder::from_pem("mailto:test@example.com");

    builder.add_audience(&sub_info.endpoint).add_expiration(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 5 * 60,
    ); // 5 минут

    let signature = builder.build(&vapid_private_key).unwrap();

    let payload_str = format!(
        "{{\"title\": \"{}\", \"body\": \"{}\"}}",
        payload.title, payload.body
    );

    match web_push::send_notification_with_custom_builder(
        &sub_info,
        payload_str.as_bytes(),
        &signature,
        Some("application/octet-stream"),
        None,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Ошибка отправки уведомления: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriptions: Subscriptions = Arc::new(Mutex::new(HashMap::new()));

    println!("🚀 Сервер запущен на http://localhost:3000");

    let data = web::Data::new(subscriptions);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/api/subscribe", web::post().to(subscribe))
            .route("/api/send-notification", web::post().to(send_notification))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
