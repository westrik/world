#!/usr/bin/env bash

# -----------------------------------------------------------------------------
# setup_test_env.sh
# - This script sets up a Postgres database to be used for integration testing
# - Expects the current operating system to be Ubuntu (e.g. via GitHub Actions)
# -----------------------------------------------------------------------------

set -euo pipefail

scripts_dir="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

<<<<<<< HEAD
bash "$scripts_dir/apt_install_dependencies.sh"
=======
>>>>>>> b4422df... [tools] add script to set up local dev environment (#180)
bash "$scripts_dir/setup_database.sh"
echo "PASSWORD_HASH_SALT='lBi5Pi6yBdtRmIXMZvs6IlmAOy3ogEivqG056hGJ6t'" >> .env
