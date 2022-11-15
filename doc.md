# RBAC

Role Based Access Control (RBAC) achieve the authorization control by grouping permissions into roles.

Example:

- Luca wanna access List of Employee Salary.
- Employee Salary requires a *read* permission on the employee-salary resource
- Luca has a *Software Engineer* role
- *Software Engineer* role has no *read* permission on that resource
- Luca's request is so denied

So the question we need to ask ourselves was:
Is Luca allowed to access the resource Employee Salary? 
This is translated in OSO as:

```rust
let user = users.find("Luca");
let resource = resources.find("employee-salary");
if oso.is_allowed(user, "read", resource) {
    Ok(())
} else {
    Err("Not allowed")
}
```
