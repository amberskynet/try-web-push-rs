use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::web;
use web_push::SubscriptionInfo;

pub type Subscriptions = Arc<Mutex<HashMap<String, SubscriptionInfo>>>;

pub fn get_app_data () -> Subscriptions {
	let subscriptions: Subscriptions = Arc::new(Mutex::new(HashMap::new()));
	subscriptions
}

pub fn get_app_data_web () -> web::Data<Subscriptions> {
	let app_data = get_app_data();
	let data = web::Data::new(app_data);
	data
}
