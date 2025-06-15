use uuid::Uuid;
use web_push::{ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient, WebPushMessageBuilder};
use crate::models::NotificationRequest;
use crate::app_data::Subscriptions;
use crate::models::SubscriptionRequest;

pub async fn do_subscribe (s: &Subscriptions , r: &SubscriptionRequest) -> Uuid {
	let mut subs = s.lock().unwrap();

	println!(" -> SubscriptionRequest: {:?}", r);

	let sub_info = SubscriptionInfo::new(
		r.endpoint.clone(),
		r.keys.p256dh.clone(),
		r.keys.auth.clone(),
	);

	let id = Uuid::new_v4();
	subs.insert(id.to_string(), sub_info);

	println!(" <- id: {:?}", id);

	id
}

pub async fn do_notification (s: &Subscriptions, r: &NotificationRequest ) -> Result<(), String >{
	let subs = s.lock().unwrap();

	println!(" -> NotificationRequest: {:?}", r);

	let Some(sub_info) = subs.get(&r.subscription_id) else {
		return Err(format!("subscription {} not found", &r.subscription_id))
	};

	let pem_content = "gei4NsFoTBLOZAFiaPdOLvJsZmUk6ufCU8w3JrBGSBk";

// 	let _public_key = "-----BEGIN PUBLIC KEY-----
// MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEa0asPq8XZCxRMmkHehMDn1eYiLhX
// ERtWokM677Q4Ah7p8dWmyRe7JXwtJQTlpBUw1K4z0RVGzQ3wFmJ/UyNqHQ==
// -----END PUBLIC KEY-----";

	// let cursor = Cursor::new(pem_content);

	println!("sub_info: {:?}", &sub_info);

	let sig_builder = VapidSignatureBuilder::from_base64(pem_content, &sub_info)
		.unwrap()
		.build()
		.unwrap();

	let mut builder = WebPushMessageBuilder::new(&sub_info);

	let payload_str = format!(
		"{{\"title\": \"{}\", \"body\": \"{}\"}}",
		r.title, r.body
	);

	builder.set_payload(ContentEncoding::Aes128Gcm, payload_str.as_bytes());
	builder.set_vapid_signature(sig_builder);

	let client = IsahcWebPushClient::new().unwrap();

	let mess_result = builder.build().unwrap();

	let result = client.send(mess_result).await;
	println!(" <- result {:?}", result);

	match result {
	    Ok(_) => {}
	    Err(e) => {
		    return Err(e.to_string())
	    },
	};

	Ok(())
}


