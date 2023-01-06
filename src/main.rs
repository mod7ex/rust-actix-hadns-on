use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    HttpResponse,
    Responder
};

use  rhai::Engine;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/add/{a}/{b}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (a, b) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("a", move || a );
    engine.register_fn("b", move || b );

    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    HttpResponse::Ok().body(format!("result : {}", result.to_string().as_str()))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (a, b) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("a", move || a );
    engine.register_fn("b", move || b );

    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    HttpResponse::Ok().body(format!("result : {}", result.to_string().as_str()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server is running on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add)
            .service(multiply)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}