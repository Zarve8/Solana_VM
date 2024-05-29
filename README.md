## Solana VM
A simple, portable, high-performance, virtualization of Solana node

SolanaVM purely written in Rust and provides basic logic for transaction execution and account load. 
### Prerequisites
* rustc
    ```sh
    rustup update stable
    ```
* npm
    ```sh
    npm install npm@latest -g
    ```
  
### Installation
1. Clone the repo
    ```sh
    git clone https://github.com/Zarve8/Solana_VM
    ```
2. Build accompanying packages
    ```sh
    sh ./shel/build_all.sh
    ```
3. Build main package 
    ```sh
    cargo build --package super-vm
    ```
4. Boostrap database
    ```sh
    cargo test --package data-manager
    ```

## Usage
### Run the project natively
The server will strat on port :8080
Base url for the node [http://localhost:8080/rpc](http://localhost:8080/rpc)
```sh
    cargo run --package super-vm
```
### Run in docker
```sh
docker compose up
```

## Test
1. Install NPM packages
    ```sh
    npm install --force
    ```
2. Test SPLToken with autotests
    ```sh
    npm run test:token
    ```
3. Run load tests 
    ```sh
    npm run test:load
    ```

## Roadmap
- [x] Implement Transaction Executor
- [x] Add Token tests
- [ ] Implement cli
- [ ] Add test suit package
