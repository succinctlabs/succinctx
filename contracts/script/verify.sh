# # Edit this array to add/remove contracts. Must have a script named Deploy${contract}.
# CONTRACTS=(CountMessenger)
# CONSTRUCTOR_ARGS=("$(cast abi-encode "constructor(address)" "0x41EA857C32c8Cb42EEFa00AF67862eCFf4eB795a")")

USAGE="Usage: ./verify.sh <contract> <chain_ids> <is_proxy> <constructor_args> \n  Example: ./verify.sh \"SuccinctGateway\" \"5 420 84531 421613\" \"true\" \"$(cast abi-encode "constructor(address)" "0x41EA857C32c8Cb42EEFa00AF67862eCFf4eB795a")\""

if [ -z "$1" ]; then
	echo $USAGE
	exit 1
fi

if [ -z "$2" ]; then
	echo $USAGE
	exit 1
fi

if [[ -z "$3" || ( "$3" != "true" && "$3" != "false" ) ]]; then
    echo "$USAGE"
    exit 1
fi

CONTRACT=$1
IFS=' ' read -r -a CHAIN_IDS <<< "$2"
IS_PROXY=$3
CONSTRUCTOR_ARGS=$4

source .env.deployments

# Create .env.deployments if it doesn't exist
if [ ! -f .env.deployments ]; then
    touch .env.deployments
fi

for chain_id in "${CHAIN_IDS[@]}"; do
	contract=$CONTRACT
	constructor_args=${CONSTRUCTOR_ARGS[$i]}
	is_proxy=${IS_PROXY[$i]}
	snake_case=$(echo $contract | sed 's/\([a-z0-9]\)\([A-Z]\)/\1_\2/g' | tr '[:lower:]' '[:upper:]')

	address_var=$(echo "${snake_case}_${chain_id}")
	address=$(echo $(eval echo "\$$address_var"))

	etherscan_key_var=$(echo 'ETHERSCAN_API_KEY_'"${chain_id}")
	etherscan_key=$(echo $(eval echo "\$$etherscan_key_var"))

	# Proxy logic
	if $is_proxy; then
		rpc_url_var=$(echo 'RPC_'"${chain_id}")
		rpc_url=$(echo $(eval echo "\$$rpc_url_var"))
		# ERC1967Proxy implementation slot
		impl=$(cast storage $address 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc --rpc-url $rpc_url)
		impl_addr=$(cast abi-decode "a()(address)" $impl)
		proxy_constructor_calldata=$(cast abi-encode "constructor(address,bytes)" $impl_addr "$constructor_args")
		echo "Verifying Proxy_${chain_id} @ $address with CONSTRUCTOR_ARGS: ${constructor_args}"
		forge verify-contract $address src/upgrade/Proxy.sol:Proxy --chain ${chain_id} --watch --constructor-args $proxy_constructor_calldata --verifier etherscan  --etherscan-api-key $etherscan_key
		address=$impl_addr
		constructor_args=""
	fi

	echo "Verifying ${contract}_${chain_id} @ $address with CONSTRUCTOR_ARGS: ${constructor_args}"
	if [ -n "$constructor_args" ]; then
		forge verify-contract $address $contract --chain ${chain_id} --watch --constructor-args $constructor_args --verifier etherscan  --etherscan-api-key $etherscan_key
	else
		forge verify-contract $address $contract --chain ${chain_id} --watch --verifier etherscan  --etherscan-api-key $etherscan_key
	fi
done
