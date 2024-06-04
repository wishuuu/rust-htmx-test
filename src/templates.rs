use askama::Template;
use crate::users::User;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "app.html")]
pub struct AppTemplate {
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    pub users: Vec<User>,
}
