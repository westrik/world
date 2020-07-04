<p align="center">
<a href="https://westrikworld.com"><img src="web-client/src/static/img/logo.png" alt="westrikworld"></a><br><br>
<a href="https://github.com/westrik/world/actions?query=workflow%3ARust"><img alt="rust build status" src="https://github.com/westrik/world/workflows/Rust/badge.svg"></a>
<a href="https://github.com/westrik/world/actions?query=workflow%3AWeb"><img alt="web build status" src="https://github.com/westrik/world/workflows/Web/badge.svg"></a>
<img alt="iOS app build status" src="https://build.appcenter.ms/v0.1/apps/15506d1c-dcc0-4479-8cf8-3aac8622f128/branches/master/badge">
<img alt="macOS app build status" src="https://build.appcenter.ms/v0.1/apps/9af600b8-a4ba-4807-a8cd-931a3965bcbc/branches/master/badge">
</p>
# westrikworld server

### DB setup

```
~/world » createdb world_app
~/world » createdb world_test
~/world » createuser world_user
~/world » psql postgres
psql (11.5)
Type "help" for help.

postgres=# alter user world_user with encrypted password 'PASSWORD';
ALTER ROLE
postgres=# grant all privileges on database world_app to world_user;
GRANT
postgres=# grant all privileges on database world_test to world_user;
GRANT
postgres=# alter database world_app set timezone to 'UTC';
ALTER DATABASE
postgres=# alter database world_test set timezone to 'UTC';
ALTER DATABASE
postgres=# \q
~/world » echo "DATABASE_URL='postgres://world_user:PASSWORD@localhost/world_app'" > .env
~/world » echo "TEST_DATABASE_URL='postgres://world_user:PASSWORD@localhost/world_test'" >> .env
```


### Local setup

```sh
~/world » echo "CORS_ORIGIN_URL=\"https://westrik.world\"" >> .env
~/world » echo "PASSWORD_HASH_SALT=\"$(gpg --gen-random --armor 0 32)\"" >> .env
~/world »
~/world » # Generate self-signed certs
~/world » sudo mkdir -p /etc/ssl/{certs,private}
~/world » sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/nginx-selfsigned.key -out /etc/ssl/certs/nginx-selfsigned.crt
~/world » sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/nginx-selfsigned-api.key -out /etc/ssl/certs/nginx-selfsigned-api.crt
~/world » sudo openssl dhparam -out /etc/ssl/certs/dhparam.pem 2048
~/world »
~/world » # Install and configure nginx
~/world » brew install nginx
~/world » sudo ln -s "$(pwd)/infra/nginx/local.conf" /usr/local/etc/nginx/world.conf
~/world » sudo ln -s "$(pwd)/infra/nginx/selfsigned.conf" /usr/local/etc/nginx/selfsigned.conf
~/world » # add 'include world.conf;':
~/world » sudo vi /usr/local/etc/nginx/nginx.conf
~/world » sudo nginx -t
~/world » sudo brew services restart nginx
```


### Adding migrations

(First, install `diesel_cli` if it is not already installed):
```sh
cargo install diesel_cli --no-default-features --feature postgres
```

Generate `up.sql` and `down.sql` in a new sub-folder.

```sh
diesel migration generate create_users
```

Put some SQL in the generated files.

Next, test the migration in both directions (and apply the migration):

```sh
diesel migration run
diesel migration redo
```

**Note**: If the migration adds a new model, it will need to be added in a few other places:


- `integration_tests::db::destroy_test_db()` - drop the new table
- `core::resource_identifier::ResourceType` (iff an API ID is needed)
