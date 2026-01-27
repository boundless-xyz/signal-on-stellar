# Signal on Stellar

Verify [Ethereum finality proofs](https://signal.boundless.network/) using the RISC Zero verifier enabled by Stellar X-Ray

Testnet deployment is live at [CDSTMIXJVKI4ZVP4QV4POCVXGMUR2CG6IE2BT65ZOGRMZ445JX6NIKTA](https://lab.stellar.org/r/testnet/contract/CDSTMIXJVKI4ZVP4QV4POCVXGMUR2CG6IE2BT65ZOGRMZ445JX6NIKTA)

> [!IMPORTANT]
> This does not enforce the link between epochs so do not treat the verified block roots as part of the canonical chain. This is for demonstration purposes only

## Usage

Generate a new testnet signer and fund

```shell
stellar keys generate submitter --network testnet --fund
```

Submit a signal proof

```shell
stellar contract invoke \
  --id CDSTMIXJVKI4ZVP4QV4POCVXGMUR2CG6IE2BT65ZOGRMZ445JX6NIKTA \
  --source-account submitter \
  --network testnet \
  -- \
  receive \
  --seal 73c457ba0df517f8a61e84093ee84e25e1bcbb62abdeaa97a63affb6aede9566f1550f771c3b80158c42fd4f8cde00a2242bbad6b0049c51f23368c2a7385302ae50a46d0c95e9c3bdcc90df32b9d91e16ebd2723f9f720a16442f5d67e36f212e13fd1d07d59570be1c1cafdaee0b18e3f06c44513e1dbb921628528121f085e27bed0e17d7209902c4391f36ed0b4374ff422a9ca208522680f7307e369fdd82fb93cf268f34d9b9c142f77554899451ee09bab6c6af7e20abcb4b0b9c2538597cc77f0b5442ba8ea3db14356bb98853bf7398012b0e70427f741604bab678c149590d051cab370dbeaef530810a399a452d91f702d2ec719ca4790ffbf4745ba4112b \
  --journal 00000000000000000000000000000000000000000000000000000000000675a0d87a8b0faf867d9521ddb155f3c58b9523367444ac5b8822169c06cd71233ee6000000000000000000000000000000000000000000000000000000000006759f7f3d03a51f303b1f1855592276e609c932cc0f91518ca4d959c38685e9f90a6900000000000000000000000000000000000000000000000000000000000675a130da8e4de5b3733b25e850b7dd457886862f7657e3ad0d54246b76ac9676396300000000000000000000000000000000000000000000000000000000000675a0d87a8b0faf867d9521ddb155f3c58b9523367444ac5b8822169c06cd71233ee60000000000000000000000000000000000000000000000000000000000ceb400
```

If the proof verifies successfully this will return `true`. Otherwise you will see an error event log.

You can 

## Deployment

A RISC0 verifier is deployed on testnet to [CANYRGDRBQPXPNEZRXDPETY7L4YVDTKTPP4QKHKZGMMHB74IR5HKIUXD](https://lab.stellar.org/r/testnet/contract/CANYRGDRBQPXPNEZRXDPETY7L4YVDTKTPP4QKHKZGMMHB74IR5HKIUXD)

Deploy a new Signal verifier with

```shell
cd contracts/signal-receiver
stellar contract build
```

```shell
stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/signal_receiver.wasm \
  --source-account submitter \
  --network testnet \
  --alias signal_verifier \
  -- \
  --risc0_verifier CANYRGDRBQPXPNEZRXDPETY7L4YVDTKTPP4QKHKZGMMHB74IR5HKIUXD \
  --initial_state 00000000000000000000000000000000000000000000000000000000000675a0d87a8b0faf867d9521ddb155f3c58b9523367444ac5b8822169c06cd71233ee6000000000000000000000000000000000000000000000000000000000006759f7f3d03a51f303b1f1855592276e609c932cc0f91518ca4d959c38685e9f90a69
```