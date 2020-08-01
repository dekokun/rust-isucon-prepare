pub fn redis() -> redis::RedisResult<()> {
    use redis::Commands;
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.set("my_key", 42)?;
    let count: i32 = con.get("my_key")?;
    assert_eq!(count, 42);

    con.set("my_key", "hoge")?;
    let count: i32 = con.get("my_key").unwrap_or(0);
    assert_eq!(count, 0);

    // use redis sorted set
    Ok(())
}
