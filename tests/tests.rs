mod data_tests {
    pub mod v1_tests {
        pub mod database_pool_tests;
        pub mod gurl_tests;
    }

    pub mod v2_tests {
        pub mod database_tests {
            pub mod database_pool_tests;
            pub mod gurl_tests;
        }
        pub mod methods_tests {
            pub mod gurl_tests;
        }
    }
}

mod api_tests {
    pub mod v1_tests {
        pub mod check_ok_tests;
        pub mod gurl_tests;
    }
    pub mod v2_tests {
        pub mod gurl_tests;
    }
}
