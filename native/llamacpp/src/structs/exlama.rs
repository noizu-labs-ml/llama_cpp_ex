// This module contains the ExLLama struct which is a wrapper around the LLama struct from the llama_cpp_rs crate.
// The ExLLama struct includes a ResourceArc to the ExLLamaRef struct, which is used to ensure safe concurrent access.
// The ExLLama struct also implements the Deref trait to allow it to be treated as a LLama object.

use std::ops::Deref;
use llama_cpp_rs::LLama;
use rustler::{NifStruct, ResourceArc};
use crate::structs::exlama_ref::ExLLamaRef;


#[derive(NifStruct)]
#[module = "LLamaCpp.Model"]
// This struct is used to create a resource that can be passed between Elixir and Rust.
pub struct ExLLama {
    pub resource: ResourceArc<ExLLamaRef>,
}

// This implementation of ExLLama creates a new instance of the ExLLama struct.
impl ExLLama {
    pub fn new(llama: LLama) -> Self {
        Self {
            resource: ResourceArc::new(ExLLamaRef::new(llama)),
        }
    }
}

// This implementation of Deref allows the ExLLama struct to be treated as a LLama object.
impl Deref for ExLLama {
    type Target = LLama;

    fn deref(&self) -> &Self::Target {
        &self.resource.0
    }
}