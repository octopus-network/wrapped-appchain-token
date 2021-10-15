# wrapped-appchain-token

This contract is used for managing the wrapped appchain token of a certain appchain in [Octopus Network](https://oct.network). Its owner is an [appchain anchor](https://github.com/octopus-network/octopus-appchain-anchor) of the corresponding appchain which will manage the `mint` and `burn` actions of the wrapped appchain token.

## Building

To build run:

```shell
./build.sh
```

## Testing

To test run:

```shell
cargo test --package wrapped-appchain-token -- --nocapture
```

## Deploy

To deploy run:

```shell
near dev-deploy
```

Init contract:

```shell
near call $WRAPPED_APPCHAIN_TOKEN new '{"owner_id":"$APPCHAIN_ANCHOR_CONTRACT_ID","premined_beneficiary":"$valid_account_id","premined_balance":"$premined_balance","metadata":{"spec":"ft-1.0.0","name":"TestToken","symbol":"TEST","decimals":18}}' --accountId $SIGNER
```

Set owner:

```bash
near call $WRAPPED_APPCHAIN_TOKEN set_owner '{"owner_id": "$APPCHAIN_ANCHOR_CONTRACT_ID"}' --accountId $SIGNER
```

Get owner:

```bash
near view $WRAPPED_APPCHAIN_TOKEN get_owner '{}'
```
