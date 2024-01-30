# Contracts

## Installation

Install Foundry

```sh
curl -L https://foundry.paradigm.xyz | bash
```

Build Contracts

```sh
forge build
```

Run Tests

```sh
forge test
```

## Addresses

`SUCCINCT_GATEWAY`=[0x6c7a05e0AE641c6559fD76ac56641778B6eCd776](https://etherscan.io/address/0x6c7a05e0AE641c6559fD76ac56641778B6eCd776)
`SUCCINCT_FEE_VAULT`=[0x296666e937b270193B960a7cEC526B351F353166](https://etherscan.io/address/0x296666e937b270193B960a7cEC526B351F353166)

If the contracts are not deployed on a chain you need, you can deploy them yourself using the deployment instructions below or by contracting Succinct.

| Chain            | Chain ID | Address                                                                                                          |
|------------------|----------|------------------------------------------------------------------------------------------------------------------|
| Mainnet          | 1        | [0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803](https://etherscan.io/address/0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803)            |
| Goerli           | 5        | [0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803](https://goerli.etherscan.io/address/0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803)     |
| Gnosis           | 100      | [0x6e4f1e9eA315EBFd69d18C2DB974EEf6105FB803](https://gnosisscan.io/address/0x6e4f1e9eA315EBFd69d18C2DB974EEf6105FB803)           |
| Holesky          | 17000    | [0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803](https://holesky.etherscan.io/address/0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803)    |
| Arbitrum         | 42161    | [0x2d27182206f0442885989626934ca9dd5290f979](https://arbiscan.io/address/0x2d27182206f0442885989626934ca9dd5290f979)             |
| Arbitrum Sepolia | 421614   | [0x2d27182206f0442885989626934ca9dd5290f979](https://sepolia.arbiscan.io/address/0x2d27182206f0442885989626934ca9dd5290f979)     |
| Sepolia          | 11155111 | [0xaea9288f0b7a8c605c4d474c56e5e74f96bfd4b3](https://sepolia.etherscan.io/address/0xaea9288f0b7a8c605c4d474c56e5e74f96bfd4b3)    |



## Deploying

Each contract has it's own deployment file in the form of `script/deploy/{Contract}.s.sol`. Inside each, there is a `Deploy{Contract}` script that will deploy the contract. This allows for programmatic cross-chain deployment of contracts using `script/deploy.sh`.

For example, to deploy SuccinctFeeVault and then SuccinctGateway on Chains 5, 420, 84531, and 421613, you would ensure that your `.env` is correctly filled out. Then run `./scripts/deploy.sh <contracts> <chain_ids>` to deploy the contracts:

```sh
./script/deploy.sh "SuccinctFeeVault SuccinctGateway" "5 420 84531 421613"
```

Note: `CREATE2_SALT` **MUST** stay the same between the entire deployment.

## Verification

Contract verification will be automatically applied during deployment. However, there may be times where verifiation fails due to general flakiness. In these cases, you can manually verify the contracts by running `./script/verify.sh <contract> <chain_ids> <is_proxy> <constructor_args>`.

For example, to verify both the proxy and implementation contract of SuccinctGateway (both of which have no `constructor_args`) on Chains 5, 420, 84531, and 421613, you would run:

```sh
./script/verify.sh "SuccinctGateway" "5 420 84531 421613" "true"
```

## Upgrading

### Upgrades via EOA

When the `TIMELOCK` is set to an EOA, you can directly upgrade the proxy contract by setting `UPGRADE_VIA_EOA=TRUE`, changing the `CREATE2_SALT` (or bytecode) of your previous implementation contract, and then running `script/deploy.sh` again.

### Upgrades via Timelock & Guardian

Timelocked upgrades take place in two parts (`schedule` and then `execute` after the Timelock's `MINIMUM_DELAY`). Doing this from from a multisig Guadian are a multi-step process:

#### Step 1: Deploy a new implementation contract

Re-deploy the new contract via `script/deploy.sh`. This will generate a new `*_IMPL` implementation contract address with the current contract code.

#### Step 2: Schedule the upgrade

Run the upgrade script with the "schedule" command (`script/upgrade.sh schedule <contracts> <chain_ids>`). This script will generate transaction hash for `Timelock.schedule()`, sign it, and upload the signature to an AWS S3 bucket. It then checks the bucket and aggregates all the signatures for the Guadian (Gnosis Safe). If enough signatures exist, the upgrade will automatically be scheduled.

#### Step 3: Execute the upgrade

Run the upgrade script with the "execute" command (`script/upgrade.sh execute <contracts> <chain_ids>`). This script will generate transaction hash for `Timelock.execute()`, sign it, and upload the signature to an AWS S3 bucket. It then checks the bucket and aggregates all the signatures for the Guadian (Gnosis Safe). If enough signatures exist, the upgrade will automatically be executed.

#### Example

To generate and sign a scheduled upgrade for SuccinctGateway on Chains 5 and 420, you would ensure that your .env is correctly filled out. Then run `script/upgrade.sh` with "schedule":

```sh
./script/upgrade.sh "schedule" "SuccinctGateway" "5 420"
```

Note: Make sure that your AWS CLI is configured correctly and that you have the necessary permissions to create, access, and delete S3 buckets.

Note: If you are using a ledger to sign the transaction, set `WALLET_TYPE=LEDGER` and specify the `MNEMONIC_INDEX` for which signer you want to use. If you are using a private key, set `WALLET_TYPE=PRIVATE_KEY` and specify the `PRIVATE_KEY` in your .env file.

After the Timelocks's `MINIMUM_DELAY` has passed, you can execute the scheduled upgrade by running the script again with "execute":

```sh
./script/upgrade.sh "execute" "SuccinctGateway" "5 420"
```
