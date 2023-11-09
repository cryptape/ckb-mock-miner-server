# ckb-mock-miner

Mock RPC server for CKB

## Usage

Start the mock CKB server:

```shell
cargo run --bin ckb-mock-miner-server -- -b 0.0.0.0:3000 -p ./mock.json
```

Test the server:

```shell
curl --location 'localhost:3000/' \
--header 'Content-Type: application/json' \ 
--data '{"jsonrpc":"2.0","method":"get_block_template","params":[],"id":1}'
```

## Test Cases

The `mock.json` file contains test scenarios:

- [x] Extension fields for dev
- [ ] Halving miner reward
- [ ] Hard fork with non-zero block version

## Verifications

The server verifies:

```
1. extra_hash : compare extra_hash in json and calc by ckb sdk
2. proposals_hash : compare proposals_hash in json and calc by ckb sdk
3. transactions_root :  compare transactions_root in json and calc by ckb sdk
4. nonce by Pow::Eaglesong 
```

## TODO

- [ ] Add more test scenarios

