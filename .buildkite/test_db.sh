#!/usr/bin/env bash

set -euo pipefail

sed -i 's/local   all             postgres                                peer/host    all             all             127.0.0.1\/32            trust/g' /etc/postgresql/10/main/pg_hba.conf
service postgresql restart
apt-get install -y sudo
echo "select 'create database westrikworld_test' where not exists (select from pg_database where datname = 'westrikworld_test')\\gexec" | sudo -u postgres psql postgres -w -h localhost  
echo "select 'create user westrikworld_user with encrypted password ''PASSWORD''' where not exists (select from pg_user where usename = 'westrikworld_user')
\\gexec" | sudo -u postgres psql postgres -w -h localhost  
sudo -u postgres psql postgres -w -h localhost -c "grant all privileges on database westrikworld_test to westrikworld_user;"
rm -rf .env
touch .env
echo "TEST_DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_test'" >> .env
echo "PASSWORD_HASH_SALT='lBi5Pi6yBdtRmIXMZvs6IlmAOy3ogEivqG056hGJ6t'" >> .env
