
# Fungible Smart Contract with Token Distribution

A Fungible Token smart contract build in rust for Near Protocol.where total supply of tokens is 1 billion tokens, named Dead Kings.

It also includes functionalities, through which admin can unlock tokens to community, founders, chess game tournment, treasury protocols.




## Languages

Languages used creating Dapp (smart contract, scripts, frontend) are

=> Rust


## Deployment

Run Script for compilation
```bash
  ./build.sh
```
Deploy it using Near CLI

### Contract Side (Only Admin/Owner)

To grant tokens to community
```bash
  dis_community
```

To grant tokens to chess tournment
```bash
  dis_chess
```

To grant tokens to founders
```bash
  dis_founders
```

To grant tokens to treasury_protocol tournment
```bash
  dis_treasury_protocol
```

To mint other tokens
```bash
  mint
```





## Authors

- [@NabeelAhmed](https://www.github.com/rajanabeeltasaddaq)

