#!/usr/bin/env bash

set -euxo pipefail

TEST_URL="http://localhost:8080/"
TEST_ITERATIONS=5
INITIAL_SLEEP_TIME=10
REQUEST_SLEEP_TIME=2

function test_service() {
  # curl --silent --output /dev/null --write-out "%{http_code}" "$1"
  curl -v "$1"
}

echo "pausing $INITIAL_SLEEP_TIME sec to wait for services to start"
sleep $INITIAL_SLEEP_TIME
echo "done pausing"

echo "service logs:"
journalctl --no-pager -u app -b
journalctl --no-pager -u nginx -b

for i in $(seq 1 $TEST_ITERATIONS)
do
  printf "making a request to %s... " "$TEST_URL"
  status_code=$(test_service $TEST_URL)
  if [[ status_code -eq 200 ]] ; then
    echo "succeeded"
    exit 0
  fi
  echo "failed (attempt $i of $TEST_ITERATIONS)"
  sleep $REQUEST_SLEEP_TIME
done

echo "all requests failed"

exit 1
