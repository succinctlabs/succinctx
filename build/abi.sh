#!/bin/bash

# exit when any command fails
set -e

echo "Generating Contract ABIs..."

# Clean old build artifacts
rm -rf ./out

# forge build: outputs normal forge .json files and json files to out/
FOUNDRY_IGNORED_ERROR_CODES='[5574,5740,1878]' forge build --contracts ./contracts --config-path ./contracts/foundry.toml --extra-output-files abi --extra-output-files evm.deployedBytecode.object --force

echo "Generated ABI!"