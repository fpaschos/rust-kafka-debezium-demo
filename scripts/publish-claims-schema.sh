#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
pushd () {
    command pushd "$@" > /dev/null
}

popd () {
    command popd "$@" > /dev/null
}

cd $SCRIPT_DIR

# Copy proto files to setter project
cp -R ../claims-schema/resources/protos ../claims-schema-setter/src/main/resources

# Run mvn register command against target schema registry
pushd ..
./gradlew claims-schema-setter:registerSchemasTask
popd
cd -