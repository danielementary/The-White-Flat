use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    println!("Hello, The White Flat!");

    let database_url = format!(
        "mariadb://{user}:{pass}@{host}:{port}/{database}",
        user = "root",
        pass = "my-secret-pw",
        host = "127.0.0.1",
        port = "3306",
        database = "mydatabase"
    );

    println!("Database URL: {}", database_url);

    let connection = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");
}
