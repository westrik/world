#!/bin/bash

TEST_URL="http://localhost:8080/"
TEST_ITERATIONS=5
INITIAL_SLEEP_TIME=10
REQUEST_SLEEP_TIME=2

function test_service() {
  curl --silent --output /dev/null --write-out "%{http_code}" "$1"
}

echo "waiting $INITIAL_SLEEP_TIME sec for services to start"
sleep $INITIAL_SLEEP_TIME

for i in $( eval echo {1..$TEST_ITERATIONS} )
do
  status_code=$(test_service $TEST_URL)
  printf "making a request to $TEST_URL... "
  if [[ status_code -eq 200 ]] ; then
    echo "succeeded"
    exit 0
  fi
  echo "failed (attempt $i of $TEST_ITERATIONS)"
  sleep $REQUEST_SLEEP_TIME
done

echo "all requests failed"
exit 1
