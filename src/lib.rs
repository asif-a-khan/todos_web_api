pub mod controllers {
    pub mod users_controller;
    pub mod todos_controller;
    pub mod refresh_tokens_controller;
    pub mod access_tokens_controller;
    pub mod api_keys_controller;
}

pub mod models {
    pub mod user;
    pub mod todo;
    pub mod refresh_token;
    pub mod access_token;
    pub mod api_key;
}

pub mod utils {
    pub mod input_validation;
    pub mod error;
    pub mod tokens;
}

pub mod routes {
    pub mod init;
    pub mod middlewares;
    pub mod users;
    pub mod todos;
}

pub mod database {
    pub mod init;
}
