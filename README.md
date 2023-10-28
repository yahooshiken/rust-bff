# rust-bff

## Develop

### Env files

```env
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
cargo build # Or simply, $ cargo b
cargo run   # Or simply, $ cargo r
```

## Deploy

I use [render](https://render.com/) for deployment.
When a branch is merged into `master` branch, the deployment will start automatically.
One single deployment typically takes from 10 to 15 minutes.
