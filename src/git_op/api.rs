use super::*;
use crate::async_task_center::{self, nvim_scheduler::api::*};
use crate::utils;

#[no_mangle]
pub extern "C" fn create_mr(param: param::MRParam) {
    let p = param::MRParamOwned::from(&param);
    let res = async_task_center::async_task(async move {
        let cli = GitlabCli::new();
        if let Err(ref e) = cli {
            nvim_error(format!("new gitlab err: {}", e)).await;
            return;
        }
        let cli = cli.unwrap();
        let op_res = cli.create_mr(&p);
        match op_res {
            Ok(mr) => {
                nvim_info(format!("MR: {}", mr.web_url)).await;
            }
            Err(e) => nvim_error(format!("create MR err: {}", e)).await,
        }
    });
    if let Err(e) = res {
        utils::nvim_error(&format!("push task err: {:?}", e));
    }
}

#[no_mangle]
pub extern "C" fn approve_mr(param: param::MRParam) {
    let p = param::MRParamOwned::from(&param);
    let task_res = async_task_center::async_task(async move {
        let cli = GitlabCli::new();
        if let Err(ref e) = cli {
            nvim_error(format!("new gitlab err: {}", e)).await;
            return;
        }
        let cli = cli.unwrap();
        let op_res = cli.approve_mr(&p);
        match op_res {
            Ok(_) => nvim_info(format!("approve mr success!")).await,
            Err(e) => nvim_error(format!("approve MR err: {}", e)).await,
        }
    });

    if let Err(e) = task_res {
        utils::nvim_error(&format!("push task err: {:?}", e));
    }
}

