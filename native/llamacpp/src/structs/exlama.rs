use std::ops::Deref;
use llama_cpp_rs::LLama;
use rustler::{NifStruct, ResourceArc};
use crate::structs::exlama_ref::ExLLamaRef;

#[derive(NifStruct)]
#[module = "LLamaCpp.Model"]
pub struct ExLLama {
    pub resource: ResourceArc<ExLLamaRef>,
}

impl ExLLama {
    pub fn new(llama: LLama) -> Self {
        Self {
            resource: ResourceArc::new(ExLLamaRef::new(llama)),
        }
    }
}


impl Deref for ExLLama {
    type Target = LLama;

    fn deref(&self) -> &Self::Target {
        &self.resource.0
    }
}
