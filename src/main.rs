use lazy_static::lazy_static;
mod mysql_sample;
mod redis_sample;
mod sqlx_sample;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test().await;
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn test() {
    println!("{}", HASHMAP.get(&0).unwrap());

    let ret = mysql_sample::mysql().expect("failed mysql crate sample");
    println!("{:?}", ret);

    let ret = sqlx_sample::sqlx().await.expect("failed sqlx crate sample");

    println!("{:?}", ret);

    let ret = redis_sample::redis().expect("failed redis crate sample");
    println!("{:?}", ret);
}

#[allow(dead_code)]
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
