allow(actor, action, resource) if
	has_permission(actor, action, resource);

actor User {}


has_permission(_actor: User, "read", repository: Repository) if
	repository.is_public;

has_permission(user: User, "commit", repository: Repository) if
	has_role(user, "maintainer", repository);

resource Repository {
	permissions = ["read", "push", "delete"];
	roles = ["contributor", "maintainer", "admin"];


	"read" if "contributor";
	"push" if "maintainer";
	"delete" if "admin";

	"maintainer" if "admin";
	"contributor" if "maintainer";
} 

has_role(user: User, roleName: String,repository: Repository) if
	role in user.roles and
	role.role = roleName and
	role.repo_id = repository.id;
