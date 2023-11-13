# ckb-mock-miner

Mock Miner RPC server for CKB

- get_block_template
- submit_block

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

## test log
successful
```angular2html
case idx:0 get_block_template , case:extension in dev chain
case idx:0 get_block_template , case:extension in dev chain
case idx:0 get_block_template , case:extension in dev chain
case idx:0 submit_block successful
case idx:1 get_block_template , case:extension in mainnet,change compact_target:0x19097873-> 0x20010000
case idx:1 submit_block successful
case idx:2 get_block_template , case:half balance in main net ,change compact_target:0x19097873-> 0x20010000
case idx:2 submit_block successful
case idx:3 get_block_template , case:half in main net chain, change compact_target:0x19097873-> 0x20010000
case idx:3 submit_block successful
===  miner test successful ====
```
failed 
```angular2html
case idx:0 get_block_template , case:extension in dev chain
case idx:0 get_block_template , case:extension in dev chain
case idx:0 submit_block failed, case:extension in dev chain ,err:verify transactions_root failed,submit block : Ok("{\"header\":{\"version\":\"0x0\",\"compact_target\":\"0x20010000\",\"timestamp\":\"0x18bb3050114\",\"number\":\"0x1\",\"epoch\":\"0x3e80001000000\",\"parent_hash\":\"0x12a852dcfeefe9dd9caf60fdcadb595a97bf56d854528319167ef8cfed26fdd2\",\"transactions_root\":\"0x1404a771282b268392ba03f58e80edb44505299c2bb49daf575838e2b6eb6dbf\",\"proposals_hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"extra_hash\":\"0x45dc633035fa8715285c0181a595f0acae7579c7f9c37af6c534586924a2e1ac\",\"dao\":\"0x08d48435a71ea12e44fea37af28623006fbf0a6f1a0000000099f54b01fbfe06\",\"nonce\":\"0xb62989456bb294ce6d90d05d85d706f5\"},\"uncles\":[],\"transactions\":[{\"version\":\"0x0\",\"cell_deps\":[],\"header_deps\":[],\"inputs\":[{\"since\":\"0x1\",\"previous_output\":{\"tx_hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"index\":\"0xffffffff\"}}],\"outputs\":[],\"outputs_data\":[],\"witnesses\":[\"0x7e0000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce801140000008883a512ee2383c01574a328f60eeccbb4d78240250000000000000020302e3131322e302d72633120283462376132336620323032332d31302d313329\"]}],\"proposals\":[],\"extension\":\"0xba0d166e69bfbb65f4ef5a18fab4d8a2742a7cdbde3e679a27dca37b65215ee7\"}") !=expected block : Ok("{\"header\":{\"version\":\"0x0\",\"compact_target\":\"0x20010000\",\"timestamp\":\"0x18bb305e0ed\",\"number\":\"0x1\",\"epoch\":\"0x3e80001000000\",\"parent_hash\":\"0x12a852dcfeefe9dd9caf60fdcadb595a97bf56d854528319167ef8cfed26fdd2\",\"transactions_root\":\"0x8404a771282b268392ba03f58e80edb44505299c2bb49daf575838e2b6eb6dbf\",\"proposals_hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"extra_hash\":\"0x45dc633035fa8715285c0181a595f0acae7579c7f9c37af6c534586924a2e1ac\",\"dao\":\"0x08d48435a71ea12e44fea37af28623006fbf0a6f1a0000000099f54b01fbfe06\",\"nonce\":\"0x0\"},\"uncles\":[],\"transactions\":[{\"version\":\"0x0\",\"cell_deps\":[],\"header_deps\":[],\"inputs\":[{\"since\":\"0x1\",\"previous_output\":{\"tx_hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"index\":\"0xffffffff\"}}],\"outputs\":[],\"outputs_data\":[],\"witnesses\":[\"0x7e0000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce801140000008883a512ee2383c01574a328f60eeccbb4d78240250000000000000020302e3131322e302d72633120283462376132336620323032332d31302d313329\"]}],\"proposals\":[],\"extension\":\"0xba0d166e69bfbb65f4ef5a18fab4d8a2742a7cdbde3e679a27dca37b65215ee7\"}"),
case idx:0 get_block_template , case:extension in dev chain
```

## TODO

- [ ] Add more test scenarios

