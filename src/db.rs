use std::collections::HashMap;

use crate::models::{Repository, RepositoryRole, User};

pub struct Db {
    repos: HashMap<String, Repository>,
    users: HashMap<String, User>,
}

impl Db {
    pub fn new() -> Self {
        let repos = {
            let mut map: HashMap<String, Repository> = HashMap::new();
            map.insert(
                "gmail".to_string(),
                Repository {
                    id: 0,
                    name: "gmail".to_string(),
                    is_public: false,
                },
            );
            map.insert(
                "react".to_string(),
                Repository {
                    id: 1,
                    name: "react".to_string(),
                    is_public: true,
                },
            );
            map.insert(
                "oso".to_string(),
                Repository {
                    id: 2,
                    name: "oso".to_string(),
                    is_public: false,
                },
            );
            map
        };

        let users = {
            let mut map: HashMap<String, User> = HashMap::new();
            map.insert(
                "larry".to_owned(),
                User {
                    roles: vec![RepositoryRole {
                        role: "admin".to_string(),
                        repo_id: 0,
                    }],
                },
            );

            map.insert(
                "anne".to_owned(),
                User {
                    roles: vec![RepositoryRole {
                        role: "maintainer".to_string(),
                        repo_id: 1,
                    }],
                },
            );

            map.insert(
                "graham".to_owned(),
                User {
                    roles: vec![RepositoryRole {
                        role: "contributor".to_string(),
                        repo_id: 2,
                    }],
                },
            );
            map
        };

        Self { repos, users }
    }
    pub fn get_repos_by_name(&self, name: &str) -> Repository {
        self.repos.get(name).unwrap().to_owned()
    }

    pub fn get_user(&self, name: &str) -> User {
        self.users.get(name).unwrap().to_owned()
    }
}
