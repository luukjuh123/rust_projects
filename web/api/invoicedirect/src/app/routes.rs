use actix_web::{get, post, web, HttpResponse};

#[get("/customers")]
async fn get_customers() -> HttpResponse {
    HttpResponse::Ok().body("All customers")
}

#[post("/customers")]
async fn add_customer(new_customer: web::Json<NewCustomer>) -> HttpResponse {
    // Implement logic to add a new customer
    HttpResponse::Created().body("Customer created")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_customers);
    cfg.service(add_customer);
}
