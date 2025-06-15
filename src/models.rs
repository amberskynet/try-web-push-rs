use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeysDto {
	pub auth: String,
	pub p256dh: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionRequest {
	pub endpoint: String,
	pub keys: KeysDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationRequest {
	pub subscription_id: String,
	pub title: String,
	pub body: String,
}
