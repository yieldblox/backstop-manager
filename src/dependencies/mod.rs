pub mod comet {
    soroban_sdk::contractimport!(file = "./src/dependencies/comet.wasm");
}

pub mod bootstrapper {
    soroban_sdk::contractimport!(file = "./src/dependencies/backstop_bootstrapper.wasm");
}
