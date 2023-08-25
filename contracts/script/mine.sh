#! /usr/bin/env bash

set -eo pipefail

USAGE="Usage: ./mine.sh <address-prefix>\n  Example: ./mine.sh \"fee00\""

if [ -z "$1" ]; then
	echo $USAGE
	exit 1
fi

guardian=0xDEd0000E32f8F40414d3ab3a830f735a3553E18e
create2=0x4e59b44847b379578588920cA78FbF26c0B4956C # Default forge create2 address

out_dir=$PWD/out
feevault_abi=$(jq -r '.bytecode.object' $out_dir/SuccinctFeeVault.sol/SuccinctFeeVault.json)

# cargo install ethabi-cli
feevault_args=$(ethabi encode params -v address ${guardian:2})
feevault_initcode=$feevault_abi$feevault_args

feevault_out=$(cast create2 -i $feevault_initcode -d $create2 --starts-with $1 | grep -E '(Address:|Salt:)')
feevault_addr=$(echo $feevault_out | awk '{print $2}' )
feevault_salt_raw=$(echo "$feevault_out" | awk -F'Salt: ' '{print $2}' | tr -d '\n')
feevault_salt=$(cast --to-uint256 "$feevault_salt_raw" )
echo -e "SuccinctFeeVault: \nAddress: $feevault_addr\nSalt: $feevault_salt"

echo -e "\nmining complete, run this command before you deploy:\nexport CREATE2_SALT=$feevault_salt"