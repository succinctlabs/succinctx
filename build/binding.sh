#!/bin/bash

# exit when any command fails
set -e

# Add to this list for each contract you want to generate bindings for. Must be in the abi folder.
CONTRACTS=(SuccinctFeeVault FunctionGateway FunctionRegistry StorageOracle ERC1967Proxy)

# Clear previous bindings so there is no leftovers (e.g. in case of rename).
rm -rf ./bindings/*

echo "Generating Go Contract Bindings..."

for contract in "${CONTRACTS[@]}"; do
	echo "Generating Binding for $contract..."
	forge inspect --contracts ./contracts --config-path ./contracts/foundry.toml $contract bytecode > ./contracts/out/$contract.sol/$contract.bin
	abigen --abi ./contracts/out/$contract.sol/$contract.abi.json --pkg bindings --type $contract --out ./bindings/$contract.go --bin ./contracts/out/$contract.sol/$contract.bin
done