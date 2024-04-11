// This module contains the ExPredictOptions struct which is a wrapper around the PredictOptions struct from the llama_cpp_rs crate.
// The ExPredictOptions struct includes various fields that are used to configure the prediction process.
// The ExPredictOptions struct also implements the From trait to allow it to be converted into a PredictOptions object.

use llama_cpp_rs::{
    options::{PredictOptions},
};
use rustler::{NifStruct};

// Wrapper around the PredictOptions struct from the llama_cpp_rs crate.
#[derive(NifStruct)]
#[module = "LLamaCpp.PredictOptions"]
pub struct ExPredictOptions {
    pub seed: i32,
    pub threads: i32,
    pub tokens: i32,
    pub top_k: i32,
    pub repeat: i32,
    pub batch: i32,
    pub n_keep: i32,
    pub top_p: f32,
    pub temperature: f32,
    pub penalty: f32,
    pub f16_kv: bool,
    pub debug_mode: bool,
    pub stop_prompts: Vec<String>,
    pub ignore_eos: bool,
    pub tail_free_sampling_z: f32,
    pub typical_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub mirostat: i32,
    pub mirostat_eta: f32,
    pub mirostat_tau: f32,
    pub penalize_nl: bool,
    pub logit_bias: String,
    pub path_prompt_cache: String,
    pub m_lock: bool,
    pub m_map: bool,
    pub prompt_cache_all: bool,
    pub prompt_cache_ro: bool,
    pub main_gpu: String,
    pub tensor_split: String,
}

// Implementation of the From trait to convert ExPredictOptions into PredictOptions.
impl From<ExPredictOptions> for PredictOptions {
    fn from(a: ExPredictOptions) -> Self {
        PredictOptions {
            seed: a.seed,
            threads: a.threads,
            tokens: a.tokens,
            top_k: a.top_k,
            repeat: a.repeat,
            batch: a.batch,
            n_keep: a.n_keep,
            top_p: a.top_p,
            temperature: a.temperature,
            penalty: a.penalty,
            f16_kv: a.f16_kv,
            debug_mode: a.debug_mode,
            stop_prompts: a.stop_prompts,
            ignore_eos: a.ignore_eos,
            tail_free_sampling_z: a.tail_free_sampling_z,
            typical_p: a.typical_p,
            frequency_penalty: a.frequency_penalty,
            presence_penalty: a.presence_penalty,
            mirostat: a.mirostat,
            mirostat_eta: a.mirostat_eta,
            mirostat_tau: a.mirostat_tau,
            penalize_nl: a.penalize_nl,
            logit_bias: a.logit_bias,
            path_prompt_cache: a.path_prompt_cache,
            m_lock: a.m_lock,
            m_map: a.m_map,
            prompt_cache_all: a.prompt_cache_all,
            prompt_cache_ro: a.prompt_cache_ro,
            main_gpu: a.main_gpu,
            tensor_split: a.tensor_split,
            token_callback: None,
        }
    }
}