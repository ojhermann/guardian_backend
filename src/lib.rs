#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod api {
    pub mod v1 {
        pub mod check_ok;
        pub mod gurl;
        pub mod paths;
    }
    pub mod v2 {
        pub mod gurl;
        pub mod gurl_most_recently_added;
        pub mod gurls;
        pub mod paths;
        pub mod methods {
            pub mod gurl;
            pub mod gurl_most_recently_added;
            pub mod gurls;
            pub mod pooled_connection;
        }
    }
}

pub mod data {
    pub mod v1 {
        pub mod database_pool;
        pub mod gurl;
    }
    pub mod v2 {
        pub mod database {
            pub mod database_pool;
            pub mod gurl;
            pub mod migration;
        }
        pub mod methods {
            pub mod gurl;
        }
    }
}

pub mod models {
    pub mod v1 {
        pub mod gurl;
        pub mod gurl_error;
    }
    pub mod v2 {
        pub mod api_error;
        pub mod database_method_error;
        pub mod database_pool_error;
        pub mod gurl;
    }
}

pub mod schema;

pub mod server {
    pub mod v1 {
        pub mod guardian_server;
    }
    pub mod v2 {
        pub mod guardian_server;
    }
}
