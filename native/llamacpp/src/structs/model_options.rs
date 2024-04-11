// This module contains the ExModelOptions struct which is a wrapper around the ModelOptions struct from the llama_cpp_rs crate.
// The ExModelOptions struct includes various fields that are used to configure the model.
// The ExModelOptions struct also implements the From trait to allow it to be converted into a ModelOptions object.

use llama_cpp_rs::{
    options::{ModelOptions},
};
use rustler::{NifStruct};

// Wrapper around the ModelOptions struct from the llama_cpp_rs crate.
#[derive(NifStruct)]
#[module = "LLamaCpp.ModelOptions"]
pub struct ExModelOptions {
    pub context_size: i32,
    pub seed: i32,
    pub n_batch: i32,
    pub f16_memory: bool,
    pub m_lock: bool,
    pub m_map: bool,
    pub low_vram: bool,
    pub vocab_only: bool,
    pub embeddings: bool,
    pub n_gpu_layers: i32,
    pub main_gpu: String,
    pub tensor_split: String,
    pub numa: bool,
}

// Implementation of the From trait to convert ExModelOptions into ModelOptions.
impl From<ExModelOptions> for ModelOptions {
    fn from(a: ExModelOptions) -> Self {
        ModelOptions {
            context_size: a.context_size,
            seed: a.seed,
            n_batch: a.n_batch,
            f16_memory: a.f16_memory,
            m_lock: a.m_lock,
            m_map: a.m_map,
            low_vram: a.low_vram,
            vocab_only: a.vocab_only,
            embeddings: a.embeddings,
            n_gpu_layers: a.n_gpu_layers,
            main_gpu: a.main_gpu,
            tensor_split: a.tensor_split,
            numa: a.numa,
        }
    }
}
