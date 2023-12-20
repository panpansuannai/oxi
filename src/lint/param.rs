use crate::utils;
use std::marker::PhantomData;

#[repr(C)]
pub struct GolangCheckParam<'a> {
    pub namespace: utils::CharPtr,
    pub filename: utils::CharPtr,
    pub package: utils::CharPtr,
    data: PhantomData<&'a ()>,
}

#[derive(Debug, Default)]
pub struct GolangCheckParamOwned {
    pub namespace: Option<String>,
    pub filename: Option<String>,
    pub package: Option<String>,
}

impl<'a> From<GolangCheckParam<'a>> for GolangCheckParamOwned {
    fn from(value: GolangCheckParam<'a>) -> Self {
        GolangCheckParamOwned {
            namespace: utils::ffi_to_str(value.namespace),
            filename: utils::ffi_to_str(value.filename),
            package: utils::ffi_to_str(value.package),
        }
    }
}

unsafe impl Send for GolangCheckParamOwned {}
