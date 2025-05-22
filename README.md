# Base contract

Base contract for [Vara Network](https://vara.network/) using Sails.

## Smart Contract Architecture

The contract works under a workspace which helps with the management of crate versions.

Contract crates:

- `client`: This is used only for testing, it generates the contract client and incorporates it its your code.
- `contract`: Here goes all the business logic of the smart contract.

    > **Note:**
    > To avoid conflicts, it is recommended that you keep the "program" name (ContractProgram), everything else, such as services, state, etc. can change.

- `tests`: Crate to tests your smart contract.
- `wasm`: This crate will compile your contract into a wasm file, that can be uploaded in the Vara Network.

### Generated files

when you compile your smart contract, it will generate some files inside an `out` directory that you will need:

- `contract_clint.rs`: File to be used to send message to this smart contract.
- `contract.id`: File that contains detailed information about the application, including:
    + *Types*: Custom types used within the program.
    + *Constructors*: The program's constructor.
    + *Services*: Commands and queries for all the services exposed by the program.
    + *Events*: Events utilized within the program.
- `contract.opt.wasm`: optimized WASM smart contract code.
- `contract.wasm`: WASM smart contract code (use only the optimized one).

## Commands

- `Compilation`: To compile the contract execute:

    ```sh
    make build
    ```
  or:
    ```sh
    cargo b -r
    ```

- `Tests`: to tests your contract code execute:

    ```sh
    make test
    ```
    or:
    ```sh
    cargo t -p tests -r
    ```
