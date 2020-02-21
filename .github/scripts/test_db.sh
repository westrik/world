set -euxo pipefail

sudo apt-get update
sudo apt-get install libpq-dev postgresql postgresql-client
sudo sed -i 's/local   all             postgres                                peer/host    all             all             127.0.0.1\/32            trust/g' /etc/postgresql/10/main/pg_hba.conf
sudo service postgresql restart
sudo -u postgres psql postgres -w -h localhost -c 'create database westrikworld_test;'
sudo -u postgres psql postgres -w -h localhost -c "create user westrikworld_user with encrypted password 'PASSWORD';"
sudo -u postgres psql postgres -w -h localhost -c "grant all privileges on database westrikworld_test to westrikworld_user;"
touch .env
echo "TEST_DATABASE_URL='postgres://westrikworld_user:PASSWORD@localhost/westrikworld_test'" >> .env
echo "PASSWORD_HASH_SALT='lBi5Pi6yBdtRmIXMZvs6IlmAOy3ogEivqG056hGJ6t'" >> .env
