use std::collections::HashMap;
use std::string::ToString;

use oso::PolarClass;
use rocket::{request::{FromRequest, self}, Request, http::Status};

#[derive(Clone, Debug, PolarClass)]
pub struct Repository {
    #[polar(attribute)]
    pub id: u32,
    #[polar(attribute)]
    pub name: String,
    #[polar(attribute)]
    pub is_public: bool,
}

pub fn get_repos_by_name(name: &str) -> Repository {
    let mut map: HashMap<&str, Repository> = HashMap::new();
    map.insert(
        "gmail",
        Repository {
            id: 0,
            name: "gmail".to_string(),
            is_public: false,
        },
    );
    map.insert(
        "react",
        Repository {
            id: 1,
            name: "react".to_string(),
            is_public: true,
        },
    );
    map.insert(
        "oso",
        Repository {
            id: 2,
            name: "oso".to_string(),
            is_public: false,
        },
    );
    map.get(name).clone().unwrap().to_owned()
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

        if let Some(name) = request.cookies().get("name") {
            let username = name.value();
            request::Outcome::Success(get_user(username))
        } else {
            request::Outcome::Failure((Status::Forbidden, "Unknown User".to_owned()))
        }
    }
}

pub fn get_user(name: &str) -> User {
    let mut map: HashMap<&str, User> = HashMap::new();
    map.insert(
        "larry",
        User {
            roles: vec![RepositoryRole {
                role: "admin".to_string(),
                repo_id: 0,
            }],
        },
    );

    map.insert(
        "anne",
        User {
            roles: vec![RepositoryRole {
                role: "maintainer".to_string(),
                repo_id: 1,
            }],
        },
    );

    map.insert(
        "graham",
        User {
            roles: vec![RepositoryRole {
                role: "contributor".to_string(),
                repo_id: 2,
            }],
        },
    );

    map.get(name).clone().unwrap().to_owned()
}
