pub mod backstop {
    soroban_sdk::contractimport!(file = "./src/dependencies/backstop.wasm");
}

pub mod emitter {
    soroban_sdk::contractimport!(file = "./src/dependencies/emitter.wasm");
}

pub mod pool_factory {
    soroban_sdk::contractimport!(file = "./src/dependencies/pool_factory.wasm");
}

pub mod pool {
    soroban_sdk::contractimport!(file = "./src/dependencies/pool.wasm");
}

pub mod comet {
    soroban_sdk::contractimport!(file = "./src/dependencies/comet.wasm");
}
