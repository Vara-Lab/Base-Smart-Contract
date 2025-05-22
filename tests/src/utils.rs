#[cfg(test)]
pub mod utils {
    use std::{
        env,
        path::PathBuf,
    };
    use sails_rs::CodeId;
    use sails_rs::{
        prelude::*,
        gtest::{calls::*, System},
        calls::Remoting
    };

    pub const ADMIN_ID: u64 = 10;
    pub const USERS_ID: [u64;3] = [11, 12, 13];
    pub const ONE_TOKEN: u128 = 1_000_000_000_000;

    pub trait Client<GTestRemoting> {
        fn new<R: Remoting + Clone>(remoting: R) -> Self;
    }

    pub fn program_space_and_code_id(admin_id: u64, min_tokens_to: Vec<u64>, tokens_to_mint: u128) -> (GTestRemoting, CodeId) {
        let program_space = create_program_space(admin_id, min_tokens_to, tokens_to_mint);
        let code_id = program_space
            .system()
            .submit_code_file(path_to_opt_wasm());

        (program_space, code_id)
    }

    pub fn create_program_space(admin_id: u64, min_tokens_to: Vec<u64>, tokens_to_mint: u128) -> GTestRemoting {
        let system = System::new();
        system.init_logger();

        min_tokens_to
            .iter()
            .for_each(|&actor_id| {
                system.mint_to(actor_id, tokens_to_mint * ONE_TOKEN);
            });

        GTestRemoting::new(system, admin_id.into())
    }

    pub fn path_to_opt_wasm() -> String {
        let wasm_path = workspace_cargo_toml_path().join("out").join("contract.opt.wasm");
        let wasm_path_str = wasm_path.to_str().unwrap();

        wasm_path_str.to_string()
    }

    pub fn workspace_cargo_toml_path() -> PathBuf {
        // Path where the file "Cargo.toml" is located (points to the root of the crate)
        // 'CARGO_MANIFEST_DIR' specifies this directory in env::var
        let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        cargo_toml_path.parent()
            .unwrap()
            .to_path_buf()
    }
}