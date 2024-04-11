// This library exposes the operations of the LLama C++ library to Elixir through Rust NIFs (Native Implemented Functions).
// The operations include creating a new LLama model, making predictions using the model, retrieving embeddings from the model, and freeing the memory used by the model.
// The `structs` module contains various structs that are used to wrap the corresponding structs from the llama_cpp_rs crate.
// The `nifs` module contains the definitions of the NIFs.
// The `ExLLamaRef` resource is registered in the `on_load` function, which is called when the NIF library is loaded.
// The `rustler::init!` macro is used to initialize the NIFs and set the `on_load` function as the load callback.

mod structs;
mod nifs;
use structs::exlama_ref::ExLLamaRef;
use nifs::{new, predict, embeddings, free_model};
use rustler::{Env, Term};


fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExLLamaRef, env);
    true
}

rustler::init!(
    "Elixir.LLamaCpp",
    [predict, new, embeddings, free_model],
    load = on_load
);
