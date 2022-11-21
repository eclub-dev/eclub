# eclub(WIP)

An modern community platform built in Rust

## Development

If you want to run locally.

`step 1: create development mysql containe`

```shell
docker run -itd --name mysql-local -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 mysql
```

`step 2: modify your development parameters`

```shell
cp .env.sample .env
```

`step 3: set you smtp configuration at *.env*`

Mail service is required when registering a user

`step 4: migrate refresh`

```shell
make migrate-refresh
```

`step 5: launch server`

```shell
cargo run
```




## Credits

[launchbadge sqlx](https://github.com/launchbadge/sqlx)

[SeaQL](https://github.com/SeaQL/sea-orm)

[tokio-rs axum](https://github.com/tokio-rs/axum)

