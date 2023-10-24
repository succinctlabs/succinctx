USAGE="Usage: ./upgrade.sh <schedule_or_execute> <contracts> <chain_ids>\n  Example: ./upgrade.sh \"schedule\" \"TelepathyRouter\" \"5 100\""

if [ -z "$1" ]; then
	echo $USAGE
	exit 1
fi

if [ -z "$2" ]; then
	echo $USAGE
	exit 1
fi

if [ -z "$3" ]; then
	echo $USAGE
	exit 1
fi

ACTION=$1
if [ "$ACTION" != "schedule" ] && [ "$ACTION" != "execute" ]; then
	echo "Invalid action: $ACTION, must be either schedule or execute"
	exit 1
fi
ACTION_CAPITALIZED=$(echo ${ACTION} | awk '{print toupper(substr($0,1,1))substr($0,2)}')

IFS=' ' read -r -a CONTRACTS <<< "$2"
IFS=' ' read -r -a CHAIN_IDS <<< "$3"

# Load environment variables from .env
source .env

# Create .env.deployments if it doesn't exist
if [ ! -f .env.deployments ]; then
    touch .env.deployments
fi

bucket_base_name="$ACTION-upgrade-sigs"

echo "Upgrade ${ACTION%?}ing for contracts ${CONTRACTS[*]} on chains ${CHAIN_IDS[*]}"

for contract in "${CONTRACTS[@]}"; do
	set -a
	source .env.deployments
	set +a
	for chain_id in "${CHAIN_IDS[@]}"; do
		rpc_url_var=$(echo 'RPC_'"${chain_id}")
		rpc_url=$(echo $(eval echo "\$$rpc_url_var"))

		## Part 1 - Generate signature
		echo "\n --- Generating $ACTION upgrade TX hash to sign for contract $contract on chain $chain_id ---"

		# Get Proxy address from deployment file with {PROXY_CONTRACT}_{CHAIN_ID} as key
		contract=${CONTRACTS[$i]}
		snake_case=$(echo $contract | sed 's/\([a-z0-9]\)\([A-Z]\)/\1_\2/g' | tr '[:lower:]' '[:upper:]')
		proxy_addr_var=$(echo "${snake_case}_${chain_id}")
		proxy_addr=$(echo $(eval echo "\$$proxy_addr_var"))

		# Get Implementation address from deployment file with {PROXY_CONTRACT}_IMPL_{CHAIN_ID} as key
		impl_addr_var=$(echo "${snake_case}_IMPL_${chain_id}")
		impl_addr=$(echo $(eval echo "\$$impl_addr_var"))

		sign_output=$(forge script UpgradeSign$ACTION_CAPITALIZED --sig "run(address,address)" $proxy_addr $impl_addr --rpc-url $rpc_url --chain $chain_id --ffi)

		# Extract signer and signature from output
		signer=$(echo "$sign_output" | awk -F' ' '/signer: address/{print $3}')
		signature=$(echo "$sign_output" | awk -F' ' '/signature: bytes/{print $3}')

		# Check if signature is set and valid
		if [ -z "$signature" ] || [[ $signature = ^0x[0-9a-fA-F]{64}$ ]]; then
			echo "Invalid signature: $signature, exiting..."
			exit 1
		fi
		
		echo "$proxy_addr_var $ACTION upgrade signer:\n$signer\nsignature:\n$signature"

		## PART 2 - Signature aggregation
		echo "\n --- Aggregating and sending signatures for contract $contract on chain $chain_id ---"

		# Define the bucket name
		bucket_name="$bucket_base_name-$(echo $contract | tr '[:upper:]' '[:lower:]')-$chain_id"

		# Check if the bucket exists
		if aws s3api head-bucket --bucket "$bucket_name" 2>/dev/null; then
			echo "Bucket $bucket_name already exists"
		else
			# If the bucket doesn't exist, create it
			echo "Creating bucket $bucket_name"
			aws s3api create-bucket --bucket "$bucket_name" --region us-west-1 --create-bucket-configuration LocationConstraint=us-west-1
		fi

		# Generate a unique filename for each signer
		signature_file_name="${proxy_addr_var}_${signer}_signature.txt"

		# Check if the signature file exists in the bucket
		if aws s3api head-object --bucket "$bucket_name" --key "$signature_file_name" 2>/dev/null; then
			echo "Signature file $signature_file_name already exists in bucket $bucket_name"
		else
			# If the signature file doesn't exist in the bucket, sign the transaction and upload the signature
			echo "Uploading signature file $signature_file_name to bucket $bucket_name"
			echo "$signature" > "$signature_file_name"
			aws s3 cp "$signature_file_name" "s3://$bucket_name/"
			rm "$signature_file_name"
		fi


		# Create a temporary directory
		tmp_dir=$(mktemp -d -t ci-XXXXXXXXXX)

		# Retrieve all the signatures from the bucket
		aws s3 sync "s3://$bucket_name/" "$tmp_dir"

		sigs=""
		for file in "$tmp_dir"/*.txt; do
			sig_content=$(cat "$file")
			# Strip '0x' from the start of the signature
			sig_content=${sig_content#0x}
			sigs+=$sig_content
		done


		# Delete the temporary directory
		rm -r "$tmp_dir"

		# Prepend '0x' to the concatenated string of signatures
		sigs="0x$sigs"
		echo "\nSending TX with aggregated signatures:\n$sigs"

		# Pass the sigs string to the forge script
		send_output=$(forge script UpgradeSend$ACTION_CAPITALIZED --sig "run(address,address,bytes)" $proxy_addr $impl_addr "$sigs" --rpc-url $rpc_url --private-key $PRIVATE_KEY --chain $chain_id --broadcast)

		# Extract success from output
		success=$(echo "$send_output" | awk -F' ' '/success: bool/{print $3}')
		if [ "$success" = "true" ]; then
			echo "Upgrade $ACTION successful for contract $contract on chain $chain_id"
		else
			echo "Upgrade $ACTION failed for contract $contract on chain $chain_id"
			continue
		fi

		# Delete the bucket
		aws s3 rb "s3://$bucket_name" --force
	done
done