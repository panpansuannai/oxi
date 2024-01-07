mod api;
pub mod param;

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

    fn create_mr(&self, p: &param::MRParamOwned) -> Result<types::MergeRequestBasic, String> {
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

    fn approve_mr(&self, p: &param::MRParamOwned) -> Result<(), String> {
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
