use actix_web::{get, web};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppMutexState {
    count: Mutex<i32>,
}

#[get("/1")]
async fn get_app_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Just Do {app_name}!")
}

#[get("/2")]
async fn get_call_count(data: web::Data<AppMutexState>) -> String {
    let mut count = data.count.lock().unwrap();
    *count += 1;
    format!("Api Call : {count}")
}

pub fn state_config(cfg: &mut web::ServiceConfig) {
    let mutex_state =  web::Data::new(AppMutexState {
        count: Mutex::new(0),
    });

    cfg
    .app_data(web::Data::new(AppState {
        app_name: String::from("Actix Web"),
    })).app_data(mutex_state.clone())
    .service(
        web::scope("/state")
            .service(get_app_state)
            .service(get_call_count)
    );
}