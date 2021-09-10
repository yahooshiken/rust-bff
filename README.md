# rust-bff

## Develop

### Env files

```
PORT=8080
ALLOWED_ORIGIN=localhost
GITHUB_USERNAME=
GITHUB_TOKEN=
TWITTER_ID=
TWITTER_BEARER_TOKEN=
QIITA_BEARER_TOKEN=
HATENA_BASIC_PASSWORD=
```

```sh
$ cargo build # Or simply, $ cargo b
$ cargo run   # Or simply, $ cargo r
```

## Deploy

```sh
$ heroku login
$ heroku container:login # With Docker Desktop running

$ docker build .

$ heroku container:push web
$ heroku container:release web
```

Ref: [Container Registry & Runtime (Docker Deploys) | Heroku Dev Center](https://devcenter.heroku.com/articles/container-registry-and-runtime)
