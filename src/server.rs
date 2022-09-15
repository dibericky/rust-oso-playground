use std::sync::{Arc, Mutex};

use rocket::get;
use rocket::http::Status;
use rocket::request::Request;
use rocket::{Build, Rocket, State};

use oso::{Oso, OsoError, PolarClass};

use crate::models::{get_repos_by_name, Repository, User};

#[catch(403)]
fn not_authorized(_: &Request) -> String {
    "Not Authorized!\n".to_string()
}

#[catch(404)]
fn not_found(_: &Request) -> String {
    "Not Found!\n".to_string()
}

#[get("/repo/<repo_name>")]
fn get_repo(oso: &State<OsoState>, repo_name: String, user: User) -> Result<String, Status> {
    let repository = get_repos_by_name(&repo_name);
    println!("{:?}", user);
    match oso.is_allowed(user, "read", repository) {
        true => Ok(format!("Welcome to repo {repo_name}")),
        false => Err(Status::Forbidden),
    }
}

#[get("/repo/<repo_name>/commit")]
fn commit_repo(oso: &State<OsoState>, repo_name: String, user: User) -> Result<String, Status> {
    let repository = get_repos_by_name(&repo_name);

    match oso.is_allowed(user, "commit", repository) {
        true => Ok(format!("Thank you for the commit on {repo_name}")),
        false => Err(Status::Forbidden),
    }
}


struct OsoState {
    oso: Arc<Mutex<Oso>>,
}

impl OsoState {
    pub fn is_allowed(&self, actor: User, action: &str, resource: Repository) -> bool {
        let guard = self.oso.lock().unwrap();
        guard
            .is_allowed(actor, action.to_string(), resource)
            .unwrap()
    }
}

pub fn oso() -> Result<Oso, OsoError> {
    let mut oso = Oso::new();

    oso.register_class(Repository::get_polar_class())?;
    oso.register_class(User::get_polar_class())?;

    oso.load_files(vec!["models.polar"])?;

    Ok(oso)
}

pub fn rocket(oso: Oso) -> Rocket<Build> {
    let oso_state = OsoState {
        oso: Arc::new(Mutex::new(oso)),
    };

    rocket::build()
        .mount("/", routes![get_repo, commit_repo])
        .manage(oso_state)
        .register("/", catchers![not_authorized, not_found])
}

pub async fn run() -> Result<(), OsoError> {
    rocket(oso()?).launch().await.unwrap();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{oso, rocket};
    use rocket::http::{Status, Cookie};
    use rocket::local::blocking::Client;

    #[test]
    fn get_repo_forbidden() {
        let oso_client = oso().unwrap();
        let client = Client::tracked(rocket(oso_client)).expect("valid rocket instance");
        let response = client
            .get("/repo/oso")
            .cookie(Cookie::new("name", "larry"))
            .dispatch();
        assert_eq!(response.status(), Status::Forbidden);
    }

    #[test]
    fn get_repo_ok() {
        let oso_client = oso().unwrap();
        let client = Client::tracked(rocket(oso_client)).expect("valid rocket instance");
        let response = client
            .get("/repo/react")
            .cookie(Cookie::new("name", "larry"))    
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn commit_repo_ok() {
        let oso_client = oso().unwrap();
        let client = Client::tracked(rocket(oso_client)).expect("valid rocket instance");
        let response = client
            .get("/repo/gmail/commit")
            .cookie(Cookie::new("name", "larry"))    
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

     #[test]
    fn commit_repo_forbidden() {
        let oso_client = oso().unwrap();
        let client = Client::tracked(rocket(oso_client)).expect("valid rocket instance");
        let response = client
            .get("/repo/gmail/commit")
            .cookie(Cookie::new("name", "graham"))    
            .dispatch();
        assert_eq!(response.status(), Status::Forbidden);
    }


}
