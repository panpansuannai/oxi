use crate::utils;
use std::ffi;
use std::marker::PhantomData;

pub type CharPtr = *const ffi::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct CreateMRParam<'a> {
    pub project: CharPtr,
    pub source: CharPtr,
    pub target: CharPtr,
    pub title: CharPtr,
    pub remove_source: i64,
    data: PhantomData<&'a ()>,
}

#[derive(Debug, Default)]
pub struct CreateMRParamOwned {
    pub project: Option<String>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub title: Option<String>,
    pub remove_source: i64,
}

impl<'a> From<&CreateMRParam<'a>> for CreateMRParamOwned {
    fn from(value: &CreateMRParam) -> Self {
        CreateMRParamOwned {
            project: utils::ffi_to_str(value.project),
            source: utils::ffi_to_str(value.source),
            target: utils::ffi_to_str(value.target),
            title: utils::ffi_to_str(value.title),
            remove_source: value.remove_source,
        }
    }
}

unsafe impl Send for CreateMRParamOwned {}
