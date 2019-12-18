# timeline server

### DB setup

```
~/timeline » createdb timeline_app
~/timeline » createdb timeline_app
~/timeline » createuser timeline_user
~/timeline » psql postgres
psql (11.5)
Type "help" for help.

postgres=# alter user timeline_user with encrypted password 'PASSWORD';
ALTER ROLE
postgres=# grant all privileges on database timeline_app to timeline_user;
GRANT
postgres=# \q
~/timeline » echo "DATABASE_URL='postgres://timeline_user:PASSWORD@localhost/timeline_app'" > .env


```

### Debug setup
```
~/timeline » echo "RUST_LOG=run_server=debug,run_worker=debug,actix_web=info" >> .env
```

