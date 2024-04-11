use llama_cpp_rs::LLama;
use llama_cpp_rs::options::{ModelOptions, PredictOptions};
use crate::structs::exlama::ExLLama;
use crate::structs::model_options::ExModelOptions;
use crate::structs::predict_options::ExPredictOptions;

#[rustler::nif(schedule = "DirtyCpu")]
fn new(path: String, model_options: ExModelOptions) -> Result<ExLLama, ()> {
    let model_options = ModelOptions::from(model_options);
    let llama = LLama::new(path.into(), &model_options).unwrap();
    Ok(ExLLama::new(llama))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn predict(llama: ExLLama, query: String, predict_options: ExPredictOptions) -> String {
    let predict_options = PredictOptions::from(predict_options);
    let result = llama.predict(query.into(), predict_options).unwrap();
    result
}

#[rustler::nif(schedule = "DirtyCpu")]
fn embeddings(llama: ExLLama, query: String, predict_options: ExPredictOptions) -> Vec<f32> {
    let mut predict_options = PredictOptions::from(predict_options);
    let result = llama
        .embeddings(query.into(), &mut predict_options)
        .unwrap();
    result
}

#[rustler::nif(schedule = "DirtyCpu")]
fn free_model(llama: ExLLama) {
    llama.free_model();
}