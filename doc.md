# RBAC

Role Based Access Control (RBAC) achieves the authorization control by grouping permissions into roles. 
Actors (for instance a User) have a role assigned on resource instead of having permission directly. 

First, we ask if the actor, having a specific role, can perform an action on a resource. Then, we check if that role has enough privilege to perform that action.


Example:

- Luca wanna *commit* to master branch of Repository "my-repo"
- commit on master requires a *maintainer* role on Repository resource
- Luca has a *contributor* role on that repository
- *contributor* role has no privileges to perform that operation
- Luca's request is so denied

So the question we need to ask ourselves was:
Is Luca allowed to perform commit action on the resource Repository? 
This is translated in OSO as:

```rust
let actor = users.find("Luca");
let action = "commit"
let resource = repositories.find("my-repo");
if oso.is_allowed(actor, action, resource) {
    Ok(())
} else {
    Err("Not allowed")
}
```

`oso.is_allowed(actor, action, resource)` matches the following directive in our OSO rules:

```
allow(actor, action, resource) if
	has_permission(actor, action, resource);

has_permission(user: User, "commit", repository: Repository) if
	has_role(user, "maintainer", repository);
```

So we can see that to *commit* on repository the user need to be a *maintainer*.

Luca is a contributor on that repository, so **has_role** will not find any match and Luca will not be allowed to commit.
