use actix_web::web::Path;

pub async fn get(Path((name,)): Path<(String,)>) -> String {
    name
}
