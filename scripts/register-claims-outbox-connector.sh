#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

pushd () {
    command pushd "$@" > /dev/null
}

popd () {
    command popd "$@" > /dev/null
}

cd $SCRIPT_DIR

pushd ..
curl -i -X POST \
  -H "Accept:application/json" \
  -H  "Content-Type:application/json" \
  http://localhost:58083/connectors/ \
  -d @conf/kafka-connect/register-claims-postgres-outbox-connector.json
popd