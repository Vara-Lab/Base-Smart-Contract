use std::{
    env,
    fs::{File, self},
    io::{BufRead, BufReader}, 
    path::PathBuf,
};
use sails_client_gen::ClientGenerator;

fn main() {
    // Build contract to get .opt.wasm
    sails_rs::build_wasm();

    if env::var("__GEAR_WASM_BUILDER_NO_BUILD").is_ok() {
        return;
    }

    let bin_path_file = File::open(".binpath").unwrap();
    let mut bin_path_reader = BufReader::new(bin_path_file);
    let mut bin_path = String::new();
    bin_path_reader.read_line(&mut bin_path).unwrap();

    let release_wasm_path = PathBuf::from(bin_path)
        .parent()
        .unwrap()
        .to_path_buf();

    if let Some(file_name) = release_wasm_path.file_name() {
        let file_name = file_name.to_string_lossy();
        if file_name == "debug" {
            return;
        }
    }

    // Path where the client will be generated 
    // 'OUT_DIR' points to a temporary directory used by the compiler 
    // to store files generated at compile time. 
    let out_dir_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Path where the file "contract.idl" will be created
    let idl_path = out_dir_path.join("contract.idl");
    let client_path = out_dir_path.join("contract_client.rs");

    // Generate the IDL file
    sails_idl_gen::generate_idl_to_file::<contract::ContractProgram>(&idl_path)
        .unwrap();

    // Generate the client code
    ClientGenerator::from_idl_path(&idl_path)
        .with_mocks("mocks")
        .generate_to(&client_path)
        .unwrap();


    let wasm_file = release_wasm_path.join("wasm.wasm");
    let wasm_opt_file = release_wasm_path.join("wasm.opt.wasm");
    let workspace_path = workspace_cargo_toml_path();
    let out_dir_path = workspace_path.join("out");
   
    if !out_dir_path.exists() {
        // create the "out" directory to store the contract idl and client
        match fs::create_dir(out_dir_path.clone()) {
            Ok(_) => {},
            Err(e) => println!("Error: {:?}", e)
        }
    }

    // Then, copies the client and idl that is in the OUT_DIR path in the "idl_and_client" directory
    fs::copy(idl_path, out_dir_path.clone().join("contract.idl"))
        .unwrap();

    fs::copy(client_path, out_dir_path.join("contract_client.rs"))
        .unwrap();

    fs::copy(wasm_file, out_dir_path.join("contract.wasm"))
        .unwrap();

    fs::copy(wasm_opt_file, out_dir_path.join("contract.opt.wasm"))
        .unwrap();
}

fn workspace_cargo_toml_path() -> PathBuf {
    // Path where the file "Cargo.toml" is located (points to the root of the crate)
    // 'CARGO_MANIFEST_DIR' specifies this directory in env::var
    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    cargo_toml_path.parent()
        .unwrap()
        .to_path_buf()
}
