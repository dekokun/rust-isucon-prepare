mod mysql_sample;
mod redis_sample;
mod sqlx_sample;

#[async_std::main]
async fn main() {
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
