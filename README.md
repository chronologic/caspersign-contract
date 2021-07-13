## caspersign-contract

This is a part of the [CasperSign](https://blog.chronologic.network/caspersign-immutable-document-signatures-on-the-blockchain-65edc4969bf0) project.

This repository holds the smart contract used by the application.

Based on https://github.com/casper-ecosystem/kv-storage-contract
(detailed description of the source contract can be found here: https://docs.casperlabs.io/en/latest/dapp-dev-guide/tutorials/kv-storage-tutorial.html)

The contract has been deployed to mainnet here: https://cspr.live/deploy/58a2cb73bf3f73249f296af7a5bd8338ac9bc55cab0eae484c810ab675d70d85

## Project overview

The CasperSign project consists of the following repositories:

- https://github.com/chronologic/caspersign-app-ui
- https://github.com/chronologic/caspersign-validator-ui
- https://github.com/chronologic/caspersign-signer-ui
- https://github.com/chronologic/caspersign-server
- https://github.com/chronologic/caspersign-contract (this repository)

## Setup

Refer to this guide to set up your environment: https://docs.casperlabs.io/en/latest/dapp-dev-guide/setup-of-rust-contract-sdk.html

## Building

Run `make build-contract`

## Deployment

Follow this guide to set up `casper-client`: https://docs.casperlabs.io/en/latest/dapp-dev-guide/deploying-contracts.html and then run:

```
casper-client put-deploy \
  --chain-name <chain_name> \
  --node-address <node_url> \
  --secret-key <path_to_secret_key> \
  --session-path ./target/wasm32-unknown-unknown/release/contract.wasm  \
  --payment-amount <amount_to_pay_for_deployment>
```

## Interacting with the contact

### Storing signature in the contract

```
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <node_url> \
  --secret-key <path_to_secret_key> \
  --session-name caspersign_contract \
  --session-entry-point store_signature \
  --session-arg="hash:string='<storage_key>'" \
  --session-arg="signature:string='<signature>'" \
  --payment-amount <amount_to_pay_for_deployment>
```

### Querying state

First, you need to get state root hash:

`casper-client get-state-root-hash --node-address <node_url>`

Then you need to find out what under what hash key the contract was stored in the deployed account:

```
casper-client query-state \
  --node-address <node_url> \
  --key <deployer_account> \
  --state-root-hash <state_root_hash>
```

Look for `key` value associated with the name `caspersign_contract`, e.g.:

```
{
    "key": "hash-sjfskdfjsdklfjsdklfjsdkfjsd",
    "name": "caspersign_contract"
},
```

Then you can query the contract for signatures:

```
casper-client query-state \
    --node-address <node_url> \
    --state-root-hash <state_root_hash> \
    --key <caspersign_contract_hash_key> \
    --query-path <storage_key>
```
