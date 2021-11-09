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

## Prepare data link for icon of fungible token metadata

According to the NEAR protocol specification, the field `icon` of `FungibleTokenMetadata` must be a data URL if present. The actual value of this field can be obtained by the following steps:

* Prepare the original icon file, the recommended file type is `svg`.
* Use the free tool [SVGOMG](https://jakearchibald.github.io/svgomg/) to optimize the original icon file.
* Encode the optimized SVG file to base64 string. It can be done by following command on linux/macOS:

```shell
base64 <optimized icon file>.svg > <base64 encoding result>.txt
```

* The value of the field `icon` should be a string with fixed prefix `data:image/svg+xml;base64,` and concatenated with the base64 encoded string of the optimized SVG file.
