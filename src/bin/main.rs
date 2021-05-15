use guardian_backend::data::database_pool;

fn main() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");
}
