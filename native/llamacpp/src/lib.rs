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
