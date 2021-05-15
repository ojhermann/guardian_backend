mod data_tests {
    pub mod database_pool_tests;
}

fn data_base_pool_tests() {
    use crate::data_tests::database_pool_tests;
    database_pool_tests::it_can_connect();
    database_pool_tests::it_panics_when_given_a_bad_url_key();
}
