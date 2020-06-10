#!/usr/bin/env bash

set -euo pipefail

# modify postgres config to trust connections from localhost
sudo sed -i 's/local   all             postgres                                peer/host    all             all             127.0.0.1\/32            trust/g' /etc/postgresql/11/main/pg_hba.conf
service postgresql restart

# create test database & user
sudo -u postgres psql postgres -w -h localhost -c 'create database westrikworld_test;'
sudo -u postgres psql postgres -w -h localhost -c "create user westrikworld_user with encrypted password 'PASSWORD';"
sudo -u postgres psql postgres -w -h localhost -c "grant all privileges on database westrikworld_test to westrikworld_user;"

# create test config file (TODO: generate)
touch .env
echo "TEST_DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_test'" >> .env
echo "PASSWORD_HASH_SALT='lBi5Pi6yBdtRmIXMZvs6IlmAOy3ogEivqG056hGJ6t'" >> .env
