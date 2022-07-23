## Config:
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


## usage: 
```
cargo run -- --from main --to prod --reference 3 --merge
```