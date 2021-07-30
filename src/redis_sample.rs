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
    con.set("my_key", "hoge")?;
    let my_key: String = con.get("my_key").unwrap_or("fuga".into());
    assert_eq!(my_key, "hoge");
    let my_key: String = con.get("not_exists").unwrap_or("fuga".into());
    assert_eq!(my_key, "fuga");

    con.zadd("rank", "1deko", 1)?;
    con.zadd("rank", "2deko", 2)?;
    con.zadd("rank", "3deko", 2)?;
    let count: i32 = con.zcard("rank")?;
    assert_eq!(count, 3);
    let rank: i32 = con.zrank("rank", "2deko")?;
    assert_eq!(rank + 1, 2);
    Ok(())
}
