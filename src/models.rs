use oso::PolarClass;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};

use crate::db::Db;

#[derive(Clone, Debug, PolarClass)]
pub struct Repository {
    #[polar(attribute)]
    pub id: u32,
    #[polar(attribute)]
    pub name: String,
    #[polar(attribute)]
    pub is_public: bool,
}

#[derive(Clone, PolarClass, Debug)]
pub struct RepositoryRole {
    #[polar(attribute)]
    pub role: String,
    #[polar(attribute)]
    pub repo_id: u32,
}
#[derive(Clone, PolarClass, Debug)]
pub struct User {
    #[polar(attribute)]
    pub roles: Vec<RepositoryRole>,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for User {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.rocket().state::<Db>() {
            Some(db) => {
                if let Some(name) = request.cookies().get("name") {
                    let username = name.value();
                    let user = db.get_user(username);
                    request::Outcome::Success(user)
                } else {
                    request::Outcome::Failure((Status::Forbidden, "Unknown User".to_owned()))
                }
            }
            None => request::Outcome::Failure((
                Status::InternalServerError,
                "Unable to retrieve Db".to_owned(),
            )),
        }
    }
}
