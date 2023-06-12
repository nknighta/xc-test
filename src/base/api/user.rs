use actix_web::{get,post,put,delete,web,Responder,HttpResponse};

#[get("/user")]
async fn get_user () -> impl Responder {
    HttpResponse::Ok().body("get_user")
}

#[post("/user")]
async fn post_user () -> impl Responder {
    HttpResponse::Ok().body("post_user")
}

#[post("/user")]
async fn update_user () -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

#[post("/user")]
async fn daete_user () -> impl Responder {
    HttpResponse::Ok().body("daete_user")
}