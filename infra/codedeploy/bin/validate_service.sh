#!/usr/bin/env bash

set -uo pipefail

TEST_ITERATIONS=5
INITIAL_SLEEP_TIME=10
REQUEST_SLEEP_TIME=2
API_TEST_URL="http://localhost:8080/"

function test_service() {
  curl --silent --output /dev/null --write-out "%{http_code}" "$1"
}

echo "pausing $INITIAL_SLEEP_TIME sec to wait for services to start"
sleep $INITIAL_SLEEP_TIME
echo "done pausing"

#echo "secrets logs:"
#journalctl -xn all --no-pager -u secrets -b

#echo "app logs:"
#journalctl -xn all --no-pager -u app -b
#
#echo "worker logs:"
#journalctl -xn all --no-pager -u worker -b
#
#echo "nginx logs:"
#journalctl -xn all --no-pager -u nginx -b

function test_service_with_retries() {
  test_url=$1
  for i in $(seq 1 $TEST_ITERATIONS)
  do
    printf "making a request to %s... " "$test_url"
    status_code=$(test_service "$test_url")
    if [[ status_code -eq 200 ]] ; then
      echo "succeeded"
      return 0
    fi
    echo "failed (attempt $i of $TEST_ITERATIONS)"
    sleep $REQUEST_SLEEP_TIME
  done
  return 1
}

echo "checking API service:"
test_service_with_retries $API_TEST_URL || (exit 1 && journalctl -xn all --no-pager -u app -b)

echo "checking worker"
if ( ! systemctl -q is-active worker ); then
  echo "worker not running! logs:"
  journalctl -xn all --no-pager -u worker -b
  exit 1
fi
