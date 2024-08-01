use axum::response::Response;

pub async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "MIDDLEWARE");
    println!();
    res
}

pub async fn check_auth() -> axum::response::Response {
    todo!()
}