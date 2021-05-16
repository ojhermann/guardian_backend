use guardian_backend::data::database_pool;

fn main() {
    dotenv::dotenv().ok();

    let dp = database_pool::get("DATABASE_URL");
    let pooled_connection = dp.get().unwrap();
}
