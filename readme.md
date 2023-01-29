## Config
add `config.json` file :

```json
{
  "token": "ghp_xxxx",
  "org_name": "rednaks",
  "is_user": true,
  "repos": [
    "MyRepo1",
    "MyRepo2"
  ]
}

```

## development

### web version
The web version is a fullstack rust app, front-end using yew.rs and backend using actix

### web-apis
```
cargo watch -x "run --bin web-apis"
```
### web-front
```
# you need first to install trunk: cargo install trunk
cd web-front
trunk serve
```


## cli usage
```
cargo run -- --from main --to prod --reference 3 --merge
```
