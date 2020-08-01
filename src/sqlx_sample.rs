pub async fn sqlx() -> Result<(), sqlx::Error> {
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
