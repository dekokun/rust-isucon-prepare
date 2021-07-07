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
use std::sync::Mutex;

struct StateTest {
    counter: i32,
}
struct AppStateWithCounter {
    counter: Mutex<StateTest>, // <- Mutex is necessary to mutate safely across threads
}
// appのdataは型で判別してる(同じ型だと同じ値がはいってくる)ので型を分ける必要がある
struct AppStateWithCounter2 {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/{id}/{name}")]
async fn index(
    web::Path((id, name)): web::Path<(u32, String)>,
    data: web::Data<AppStateWithCounter>,
    data2: web::Data<AppStateWithCounter2>,
) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter = StateTest {
        counter: counter.counter + 1,
    }; // <- access counter inside MutexGuard
    let mut counter2 = data2.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter2 += 2; // <- access counter inside MutexGuard
    format!(
        "Hello {}! id:{}\nRequest number: {}, number2: {}",
        name, id, counter.counter, counter2
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(StateTest { counter: 0 }),
    });
    let counter2 = web::Data::new(AppStateWithCounter2 {
        counter: Mutex::new(0),
    });
    test().await;
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(counter2.clone())
            .service(index)
    })
    .workers(10)
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
