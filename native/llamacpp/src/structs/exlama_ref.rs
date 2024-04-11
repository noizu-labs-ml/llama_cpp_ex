// This module contains the ExLLamaRef struct which is a wrapper around the LLama struct from the llama_cpp_rs crate.
// The ExLLamaRef struct is used to ensure safe concurrent access to the LLama object.
// The ExLLamaRef struct is also marked as Send and Sync, allowing it to be shared across threads.

use llama_cpp_rs::{
    LLama,
};

// This struct is used to create a reference to the LLama object.
// The ExLLamaRef struct is used to ensure safe concurrent access to the LLama object.
// The ExLLamaRef struct is also marked as Send and Sync, allowing it to be shared across threads.
pub struct ExLLamaRef(pub LLama);

// This implementation of ExLLamaRef creates a new instance of the ExLLamaRef struct.
impl ExLLamaRef {
    pub fn new(llama: LLama) -> Self {
        Self(llama)
    }
}

// The Send trait is implemented for ExLLamaRef to allow ownership of ExLLamaRef instances to be transferred across threads.
// This is safe because all operations on LLama are thread-safe.
unsafe impl Send for ExLLamaRef {}

// The Sync trait is implemented for ExLLamaRef to allow shared references (&ExLLamaRef) to be used from multiple threads concurrently.
// This is safe because all operations on LLama are thread-safe.
unsafe impl Sync for ExLLamaRef {}