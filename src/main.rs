#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let ret = mysql();
    println!("{:?}", ret);
    let ret = sqlx().await;

    println!("{:?}", ret);

    let ret = redis();
    println!("{:?}", ret);
    Ok(())
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn redis() -> redis::RedisResult<()> {
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

async fn sqlx() -> Result<(), sqlx::Error> {
    use sqlx::prelude::*;
    use sqlx::*;
    let mut conn = MySqlConnection::connect("mysql://isucon:isucon@localhost:3306/isucon").await?;
    let _insert_count = sqlx::query!("INSERT INTO payment (customer_id, amount, account_name) VALUES (1, 1, 'hoge1'), (2, 2, null), (3, 3, null)").execute(&mut conn).await?;
    let fetch_all_rows = sqlx::query!("SELECT * from payment where customer_id = ?", 1)
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(fetch_all_rows[0].customer_id, 1);
    assert_eq!(fetch_all_rows[0].amount, 1);
    assert_eq!(fetch_all_rows[0].account_name.as_ref().unwrap(), "hoge1");

    let fetch_one_row = sqlx::query!("SELECT * from payment where customer_id = ?", 1)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(fetch_all_rows[0].amount, fetch_one_row.amount);
    Ok(())
}

fn mysql() -> std::result::Result<String, mysql::error::Error> {
    use mysql::prelude::*;
    use mysql::*;
    let url = "mysql://isucon:isucon@localhost:3306/isucon";

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )",
    )?;

    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| {
            params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }
        }),
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    assert_eq!(payments, selected_payments);
    Ok("OK".into())
}
