# oso Rust Quickstart

Follow along [here](https://docs.osohq.com/getting-started/quickstart.html).

## Instructions

1. Clone this repository.
2. Run the server: `cargo run`

## Oso Cloud

To insert a fact, install and connect with the Oso Cloud CLI:
```
curl -L https://cloud.osohq.com/install.sh | bash
```

Copy to clipboard
```
export OSO_AUTH="[YOUR_API_KEY]"
```
Copy to clipboard

```
oso-cloud tell has_permission User:bob read Organization:acme
```
