#!/usr/bin/env bash

set -uo pipefail

TEST_ITERATIONS=5
INITIAL_SLEEP_TIME=10
REQUEST_SLEEP_TIME=2
API_TEST_URL="http://localhost:8080/"
WORKER_TEST_URL="http://localhost:8090/"

function test_service() {
  curl --output /dev/null --write-out "%{http_code}" "$1"
}

echo "pausing $INITIAL_SLEEP_TIME sec to wait for services to start"
sleep $INITIAL_SLEEP_TIME
echo "done pausing"

echo "secrets logs:"
journalctl -xn all --no-pager -u secrets -b

echo "app logs:"
journalctl -xn all --no-pager -u app -b

echo "worker logs:"
journalctl -xn all --no-pager -u worker -b

echo "nginx logs:"
journalctl -xn all --no-pager -u nginx -b

for test_url in $API_TEST_URL $WORKER_TEST_URL; do
  for i in $(seq 1 $TEST_ITERATIONS)
  do
    printf "making a request to %s... " "$test_url"
    status_code=$(test_service "$test_url")
    if [[ status_code -eq 200 ]] ; then
      echo "succeeded"
      exit 0
    fi
    echo "failed (attempt $i of $TEST_ITERATIONS)"
    sleep $REQUEST_SLEEP_TIME
  done
done

echo "all requests failed"

exit 1
