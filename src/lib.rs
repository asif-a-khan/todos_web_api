pub mod controllers {
    pub mod users_controller;
    pub mod todos_controller;
}

pub mod models {
    pub mod user;
    pub mod todo;
}

pub mod utils {
    pub mod input_validation;
    pub mod error;
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
