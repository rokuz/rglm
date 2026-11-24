use anyhow::{Context, bail};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct Workspace {
    target_dir: PathBuf,
    publishable: Vec<(String, String)>,
}

pub fn package_crates(root: &Path, args: &[String]) -> anyhow::Result<()> {
    let mut out_dir = None;
    let mut allow_dirty = false;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = rest.next().context("--out-dir needs a directory")?;
                out_dir = Some(PathBuf::from(value));
            }
            "--allow-dirty" => allow_dirty = true,
            other => bail!(
                "unknown package-crates option `{other}` (expected `--out-dir DIR` or `--allow-dirty`)"
            ),
        }
    }

    let workspace = workspace(root)?;
    if workspace.publishable.is_empty() {
        bail!("the workspace has no publishable crate");
    }
    let out_dir = match out_dir {
        Some(path) => std::path::absolute(&path)
            .with_context(|| format!("cannot resolve {}", path.display()))?,
        None => workspace.target_dir.join("dist"),
    };
    fs::create_dir_all(&out_dir).with_context(|| format!("cannot create {}", out_dir.display()))?;

    let mut command = Command::new("cargo");
    command.current_dir(root).args(["package"]);
    if allow_dirty {
        command.arg("--allow-dirty");
    }
    for (name, _) in &workspace.publishable {
        command.args(["-p", name]);
    }
    let status = command.status().context("failed to spawn cargo package")?;
    if !status.success() {
        bail!("cargo package failed with {status}");
    }

    for (name, version) in &workspace.publishable {
        let file = format!("{name}-{version}.crate");
        let source = workspace.target_dir.join("package").join(&file);
        let destination = out_dir.join(&file);
        fs::copy(&source, &destination)
            .with_context(|| format!("cannot copy {}", source.display()))?;
        let size = fs::metadata(&destination)
            .with_context(|| format!("cannot read {}", destination.display()))?
            .len();
        println!("{} {size}", destination.display());
    }
    Ok(())
}

fn workspace(root: &Path) -> anyhow::Result<Workspace> {
    let output = Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to spawn cargo metadata")?;
    if !output.status.success() {
        bail!(
            "cargo metadata failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let metadata: Value =
        serde_json::from_slice(&output.stdout).context("cannot parse cargo metadata")?;
    let target_dir = metadata["target_directory"]
        .as_str()
        .map(PathBuf::from)
        .context("cargo metadata has no target directory")?;
    let packages = metadata["packages"]
        .as_array()
        .context("cargo metadata has no packages")?;
    let mut publishable = Vec::new();
    for package in packages {
        let (Some(name), Some(version)) = (package["name"].as_str(), package["version"].as_str())
        else {
            bail!("cargo metadata has a package without a name or a version");
        };
        if package["publish"].is_null() {
            publishable.push((name.to_string(), version.to_string()));
        }
    }
    publishable.sort();
    Ok(Workspace {
        target_dir,
        publishable,
    })
}
