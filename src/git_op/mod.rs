pub mod param;

use crate::{task_center, utils};
use gitlab::{api::projects::merge_requests::CreateMergeRequest, api::Query, types, Gitlab};
use std::env;

#[derive(Debug)]
pub struct GitlabCli {
    inner: gitlab::Gitlab,
}

impl GitlabCli {
    pub fn new() -> Result<GitlabCli, String> {
        let (host, key) = (
            env::var("OXI_GIT_HOST").map_err(|e| e.to_string())?,
            env::var("OXI_GIT_KEY").map_err(|e| e.to_string())?,
        );
        Gitlab::new(host, key)
            .map(|c| GitlabCli { inner: c })
            .map_err(|e| e.to_string())
    }

    pub fn create_mr(
        &self,
        p: &param::CreateMRParamOwned,
    ) -> Result<types::MergeRequestBasic, String> {
        CreateMergeRequest::builder()
            .project(p.project.to_owned().ok_or("project empty")?)
            .title(p.title.to_owned().ok_or("title empty")?)
            .source_branch(p.source.to_owned().ok_or("source empty")?)
            .target_branch(p.target.to_owned().ok_or("target empty")?)
            .remove_source_branch(p.remove_source > 0)
            .build()
            .map_err(|e| -> String { e.to_string() })?
            .query(&self.inner)
            .map_err(|e| -> String { e.to_string() })
    }

    pub fn approve_mr(&self, p: &param::CreateMRParamOwned) -> Result<(), String> {
        let source = p.source.to_owned().ok_or("url empty")?;
        let id: u64 = source
            .parse()
            .map_err(|_| format!("get source id {:?} err", source))?;
        gitlab::api::projects::merge_requests::ApproveMergeRequest::builder()
            .project(p.project.to_owned().ok_or("project empty")?)
            .merge_request(id)
            .build()
            .map_err(|e| -> String { e.to_string() })?
            .query(&self.inner)
            .map_err(|e| -> String { e.to_string() })
    }
}

#[no_mangle]
pub extern "C" fn create_mr(param: param::CreateMRParam) {
    let p = param::CreateMRParamOwned::from(&param);
    let task_res = task_center::push_task(Box::new(move || {
        let cli = GitlabCli::new();
        if let Err(ref e) = cli {
            utils::nvim_error(&format!("new gitlab err: {}", e));
            return;
        }
        let cli = cli.unwrap();
        let _ = cli
            .create_mr(&p)
            .map(|mr| {
                utils::nvim_info(format!("MR: {}", mr.web_url));
            })
            .map_err(|e| {
                utils::nvim_error(&format!("create MR err: {}", e));
            });
    }));

    if let Err(e) = task_res {
        utils::nvim_error(&format!("push task err: {:?}", e));
    }
    let _ = nvim_oxi::api::command("lua vim.fn.ScheduleTask()");
}

#[no_mangle]
pub extern "C" fn approve_mr(param: param::CreateMRParam) {
    let p = param::CreateMRParamOwned::from(&param);
    let task_res = task_center::push_task(Box::new(move || {
        let cli = GitlabCli::new();
        if let Err(ref e) = cli {
            utils::nvim_error(&format!("new gitlab err: {}", e));
            return;
        }
        let cli = cli.unwrap();
        let _ = cli
            .approve_mr(&p)
            .map(|_| {
                utils::nvim_info(format!("approve mr success!"));
            })
            .map_err(|e| {
                utils::nvim_error(&format!("approve MR err: {}", e));
            });
    }));

    if let Err(e) = task_res {
        utils::nvim_error(&format!("push task err: {:?}", e));
    }
    let _ = nvim_oxi::api::command("lua vim.fn.ScheduleTask()");
}
