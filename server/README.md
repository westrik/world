# westrikworld server

### DB setup

```
~/westrikworld » createdb westrikworld_app
~/westrikworld » createuser westrikworld_user
~/westrikworld » psql postgres
psql (11.5)
Type "help" for help.

postgres=# alter user westrikworld_user with encrypted password 'PASSWORD';
ALTER ROLE
postgres=# grant all privileges on database westrikworld_app to westrikworld_user;
GRANT
postgres=# \q
~/westrikworld » echo "DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_app'" > .env
```


### Local setup

```
~/westrikworld » echo "CORS_ORIGIN_URL=\"http://westrik.world\"" >> .env
~/westrikworld » echo "PASSWORD_HASH_SALT=\"$(gpg --gen-random --armor 0 32)\"" >> .env
```


### [setup] install diesel cli

```
cargo install diesel_cli --no-default-features --features postgres
```
