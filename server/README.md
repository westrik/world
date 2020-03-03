# westrikworld server

### DB setup

```
~/westrikworld » createdb westrikworld_app
~/westrikworld » createdb westrikworld_test
~/westrikworld » createuser westrikworld_user
~/westrikworld » psql postgres
psql (11.5)
Type "help" for help.

postgres=# alter user westrikworld_user with encrypted password 'PASSWORD';
ALTER ROLE
postgres=# grant all privileges on database westrikworld_app to westrikworld_user;
GRANT
postgres=# grant all privileges on database westrikworld_test to westrikworld_user;
GRANT
postgres=# alter database westrikworld_app set timezone to 'UTC';
ALTER DATABASE
postgres=# alter database westrikworld_test set timezone to 'UTC';
ALTER DATABASE
postgres=# \q
~/westrikworld » echo "DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_app'" > .env
~/westrikworld » echo "TEST_DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_test'" >> .env
```


### Local setup

```
~/westrikworld » echo "CORS_ORIGIN_URL=\"http://westrik.world\"" >> .env
~/westrikworld » echo "PASSWORD_HASH_SALT=\"$(gpg --gen-random --armor 0 32)\"" >> .env
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
