// This module contains the NIF (Native Implemented Function) definitions for the LLamaCpp project.
// NIFs are functions that are implemented in Rust and can be called from Elixir.
// The functions in this module include `new`, `predict`, `embeddings`, and `free_model`.
// The `new` function is used to create a new instance of the ExLLama struct.
// The `predict` function is used to make predictions using the LLama model.
// The `embeddings` function is used to get the embeddings from the LLama model.
// The `free_model` function is used to free the memory used by the LLama model.

use llama_cpp_rs::LLama;
use llama_cpp_rs::options::{ModelOptions, PredictOptions};
use crate::structs::exlama::ExLLama;
use crate::structs::model_options::ExModelOptions;
use crate::structs::predict_options::ExPredictOptions;


#[rustler::nif(schedule = "DirtyCpu")]
// This function creates a new instance of the ExLLama struct.
fn new(path: String, model_options: ExModelOptions) -> Result<ExLLama, ()> {
    let model_options = ModelOptions::from(model_options);
    let llama = LLama::new(path.into(), &model_options).unwrap();
    Ok(ExLLama::new(llama))
}

#[rustler::nif(schedule = "DirtyCpu")]
// This function makes predictions using the LLama model.
fn predict(llama: ExLLama, query: String, predict_options: ExPredictOptions) -> String {
    let predict_options = PredictOptions::from(predict_options);
    let result = llama.predict(query.into(), predict_options).unwrap();
    result
}

#[rustler::nif(schedule = "DirtyCpu")]
// This function gets the embeddings from the LLama model.
fn embeddings(llama: ExLLama, query: String, predict_options: ExPredictOptions) -> Vec<f32> {
    let mut predict_options = PredictOptions::from(predict_options);
    let result = llama
        .embeddings(query.into(), &mut predict_options)
        .unwrap();
    result
}

#[rustler::nif(schedule = "DirtyCpu")]
// This function frees the memory used by the LLama model.
fn free_model(llama: ExLLama) {
    llama.free_model();
}