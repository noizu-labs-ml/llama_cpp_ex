use llama_cpp_rs::{
    LLama,
};

pub struct ExLLamaRef(pub LLama);

impl ExLLamaRef {
    pub fn new(llama: LLama) -> Self {
        Self(llama)
    }
}


unsafe impl Send for ExLLamaRef {}
unsafe impl Sync for ExLLamaRef {}
