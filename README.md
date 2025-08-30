[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)]( https://gitpod.io/new/#https://github.com/Vara-Lab/Base-Smart-Contract.git)

# Base contract

Base contract for [Vara Network](https://vara.network/) using Sails.

## ⚙️ Settings

### Rust: You need to have rust 1.89 or newer to be able to compile your contract:

> Note: GitPod will automatically execute these commands.

- Install necessary rust version and components:

```bash
rustup install 1.89.0
rustup default 1.89.0
rustup target add wasm32v1-none
```

- Install the wasm-opt for contract compilations:

```bash 
sudo apt install binaryen
```

## 📁 Smart Contract Architecture

The contract works under a workspace which helps with the management of crate versions.

Contract crates:

- `client`: This is used only for testing, it generates the contract client and incorporates it its your code.
- `app`: Here goes all the business logic of the smart contract.

    > **Note:**
    > To avoid conflicts, it is recommended that you keep the "program" name (ContractProgram), everything else, such as services, state, etc. can change.

### 📄 Generated files

when you compile your smart contract, it will generate some files inside an `target/wasm32-gear/release` directory that you will need:

- `contract_clint.rs`: File to be used to send message to this smart contract.
- `contract.idl`: File that contains detailed information about the application, including:
    + *Types*: Custom types used within the program.
    + *Constructors*: The program's constructor.
    + *Services*: Commands and queries for all the services exposed by the program.
    + *Events*: Events utilized within the program.
- `contract.opt.wasm`: optimized WASM smart contract code.
- `contract.wasm`: WASM smart contract code (use only the optimized one).

## 💻 Usage

- 🏗️ `Compilation`: To build the contract execute:

    ```sh
    cargo b -r
    ```

- ✅ `Tests`: to tests your contract code you can execute:
    - To do unit testing with mocks (and cfg(test) in each service):
        ```sh
        cargo test -p contract-app
        ```

    - To run tests with gtest:
        ```sh
        cargo test -r
        ```

## Gitpod

You can also program your smart contract from GitPod by clicking the following button (The necessary packages and dependencies will be installed automatically):

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/Base-Smart-Contract.git" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

## Deploy the Contract on the IDEA Platform and Interact with Your Contract

### Step 1: Compile and Deploy the Smart Contract (On gitpod or local environment)

#### Compile the smart contract by running the following command:

```bash
cargo b -r
```

Once the compilation is complete, locate the `contract.opt.wasm` and `contract.idl` file in the `target/wasm32-gear/release` directory.

### Step 2: Download Your Substrate Wallet.

1. To interact with the Gear IDEA and deploy your contract, you will need to download a wallet extension such as [Polkadot-JS](https://polkadot.js.org/extension/), [Talisman](https://talisman.xyz/), or [Subwallet](https://subwallet.app/) to interact with Substrate-based chains.

<div align="center">
  <img src="https://polkadot.js.org/extension/extension-overview.png" alt="Polkadot-JS Extension">
</div>

### Step 3: Deploy Your Contract on Vara Network

1. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
2. Connect your Substrate wallet to Gear IDEA.
3. Upload the `contract.opt.wasm` and `contract.idl` files by clicking the "Upload Program" button.

## Standards: [Standards](https://github.com/gear-foundation/standards.git)