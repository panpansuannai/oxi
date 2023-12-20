mod param;

use param::*;

use crate::async_task_center::{self, nvim_scheduler::api};
use std::process::Command;

#[no_mangle]
pub extern "C" fn go_static_check(param: GolangCheckParam) {
    let p = GolangCheckParamOwned::from(param);
    let _ = async_task_center::async_task(async move {
        let mut cmd = &mut Command::new("staticcheck");
        if let Some(ref pkg) = p.package {
            cmd = cmd.arg(pkg.as_str());
        }
        let output = cmd.output();
        if let Err(e) = output {
            api::nvim_error(format!("run cmd err: {}", e.to_string())).await;
            return;
        }
        let output = output.unwrap();
        let output = std::str::from_utf8(output.stdout.as_ref());
        if let Err(e) = output {
            api::nvim_error(e.to_string()).await;
        }
        let diagnostic = parse_diagnostic(output.unwrap());
        if let Err(e) = setup_diagnostic(
            p.namespace.clone().unwrap_or("default".to_string()),
            p.filename.clone().unwrap_or("".to_string()),
            diagnostic.await,
        )
        .await
        {
            api::nvim_error(e).await;
        }
    });
}

async fn parse_diagnostic(output: &str) -> Vec<(String, i64, i64, String)> {
    let re = regex::Regex::new(r"(\S*):(\d*):(\d*):\s*(.*)");
    if let Err(e) = re {
        api::nvim_error(e.to_string()).await;
        return Vec::new();
    }

    let mut caps: Vec<(String, i64, i64, String)> = Vec::new();
    let re = re.unwrap();
    for line in output.lines() {
        let captures = re.captures(line);
        let _ = captures.map(|cap| {
            let filename = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            let line = cap
                .get(2)
                .map(|l| -> i64 { l.as_str().to_string().parse().unwrap_or(0) })
                .unwrap_or(0);
            let col = cap
                .get(3)
                .map(|l| -> i64 { l.as_str().to_string().parse().unwrap_or(0) })
                .unwrap_or(0);
            let msg = cap.get(4).map(|m| m.as_str()).unwrap_or("").to_string();
            caps.push((filename, line, col, msg));
        });
    }
    return caps;
}

async fn setup_diagnostic(
    namesapce: String,
    filename: String,
    diagnostic: Vec<(String, i64, i64, String)>,
) -> Result<(), String> {
    let mut dnst: Vec<u8> = vec![b'{'];
    for d in diagnostic.iter() {
        if filename.contains(&d.0) {
            dnst.extend_from_slice(
                format!(
                    r#"{}lnum={}, col={}, message="{}"{}"#,
                    "{",
                    d.1 - 1,
                    d.2,
                    d.3,
                    "},"
                )
                .as_bytes(),
            )
        }
    }
    if dnst.len() > 1 {
        dnst.pop();
    }
    dnst.push(b'}');
    let dnst = String::from_utf8(dnst).map_err(|e| e.to_string())?;
    let _ = api::nvim_exec_lua(format!(
        r#"
    local buf = vim.api.nvim_get_current_buf()
    local ns = vim.api.nvim_create_namespace("{0}")
    vim.diagnostic.reset(ns, 0)
    vim.diagnostic.set(ns, buf, {1}, nil)
    "#,
        namesapce, dnst,
    ))
    .await;
    Ok(())
}
