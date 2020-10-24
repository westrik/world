#!/usr/bin/env bash

# -----------------------------------------------------------------------------
# setup_database.sh
# -----------------------------------------------------------------------------

set -euo pipefail

scripts_dir="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
project_root_dir=$(dirname "$scripts_dir")

sudo_prefix=""
pg_user=""
if [[ "$OSTYPE" != "darwin"* ]]; then
  sudo_prefix="sudo -u postgres "
  pg_user="-U postgres"
fi


function run_sql() {
  sql=$1
  $sudo_prefix psql postgres $pg_user -w -c "$sql"
}

function create_database() {
  database_name="$1"
  database_user="$2"
  database_password="$3"

  if [ "$( $sudo_prefix psql postgres $pg_user -w -tAc "SELECT 1 FROM pg_database WHERE datname='$database_name'" )" = '1' ]; then
    echo "Database '$database_name' already exists, skipping"
    return
  fi

  run_sql "create database $database_name;"
  run_sql "create user $database_user with encrypted password '$database_password}';"
  run_sql "grant all privileges on database $database_name to $database_user;"
  run_sql "alter database $database_name set timezone to 'UTC';"
}

# adapted from: https://gist.github.com/cdown/1163649
function urlencode() {
    old_lc_collate=${LC_COLLATE:-}
    LC_COLLATE=C

    local length="${#1}"
    for (( i = 0; i < length; i++ )); do
        local c="${1:$i:1}"
        case $c in
            [a-zA-Z0-9.~_-]) printf '%s' "$c" ;;
            *) printf '%%%02X' "'$c" ;;
        esac
    done

    LC_COLLATE=$old_lc_collate
}

touch "$project_root_dir/.env"

if [[ "$OSTYPE" != "darwin"* ]]; then
  # modify postgres config to trust connections from localhost
  cat /etc/postgresql/12/main/pg_hba.conf
  sudo sed -i 's/local   all             postgres                                peer/host    all             all             127.0.0.1\/32            trust/g' /etc/postgresql/12/main/pg_hba.conf
  cat /etc/postgresql/12/main/pg_hba.conf
  service postgresql restart
fi

db_name="world_app"
user="world_app"
password="$(gpg --gen-random --armor 0 32)"
urlencoded_password=$(urlencode "$password")
create_database $db_name $user "$password"
echo "DATABASE_URL='postgres://$user:$urlencoded_password@localhost/$db_name'" >> .env

db_name="world_test"
user="world_test"
password="$(gpg --gen-random --armor 0 32)"
urlencoded_password=$(urlencode "$password")
create_database $db_name $user "$password"
echo "TEST_DATABASE_URL='postgres://$user:$urlencoded_password@localhost/$db_name'" >> .env
