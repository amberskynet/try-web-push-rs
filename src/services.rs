use actix_web::{HttpResponse, Responder, web};
use crate::domain::do_notification;
use crate::domain::do_subscribe;
use crate::models::NotificationRequest;
use crate::app_data::Subscriptions;
use crate::models::SubscriptionRequest;

pub async fn hey() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

pub async fn subscribe(
	payload: web::Json<SubscriptionRequest>,
	data: web::Data<Subscriptions>
) -> impl Responder {
	let id = do_subscribe(&data, &payload)
		.await;

	println!("id: {:?}", id);

	let r = NotificationRequest{
		subscription_id: id.to_string(),
		title: "Title message".to_string(),
		body: "Message ok".to_string(),
	};

	let _ = do_notification(&data, &r).await;

	HttpResponse::Ok().body(id.to_string())
}

pub async fn notify (
	payload: web::Json<NotificationRequest>,
	data: web::Data<Subscriptions>
) -> impl Responder {

	let _ = do_notification(&data, &payload).await;

	HttpResponse::Ok().body("")
}
