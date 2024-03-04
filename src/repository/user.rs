use sqlx::PgPool;

pub async fn get_user_by_email(conn: PgPool, email: String) -> anyhow::Result<()> {
    return sqlx::query_file!("src/repository/sql/select_user_by_email.sql", email)
        .fetch_one(&mut conn)
        .await?;
}
