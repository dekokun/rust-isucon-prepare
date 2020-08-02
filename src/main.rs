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

#[async_std::main]
async fn main() {

    println!("{}", HASHMAP.get(&0).unwrap())

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
