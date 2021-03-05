use crate::web::scopes::NameScope;

pub async fn get(scope: NameScope) -> String {
    scope.name
}
