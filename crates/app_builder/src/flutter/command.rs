use crate::flutter::FlutterVersion;
use fastforge_core::BuildError;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct FlutterCommand<'a> {
    environment: Option<&'a HashMap<String, String>>,
}

impl<'a> FlutterCommand<'a> {
    pub fn new(environment: Option<&'a HashMap<String, String>>) -> Self {
        Self { environment }
    }

    pub fn clean(&self) -> Result<(), BuildError> {
        let mut cmd = self.base_command()?;
        cmd.arg("clean");
        let output = cmd
            .output()
            .map_err(|e| BuildError::Io(format!("Failed to execute flutter clean: {}", e)))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(BuildError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub fn build(&self, subcommand: &str, arguments: &[String]) -> Result<i32, BuildError> {
        let mut cmd = self.base_command()?;
        cmd.arg("build").arg(subcommand).args(arguments);
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        let status = cmd
            .status()
            .map_err(|e| BuildError::Io(format!("Failed to execute flutter build: {}", e)))?;
        Ok(status.code().unwrap_or(-1))
    }

    pub fn build_with_echo(
        &self,
        subcommand: &str,
        arguments: &[String],
    ) -> Result<i32, BuildError> {
        eprintln!("$ flutter build {} {}", subcommand, arguments.join(" "));
        self.build(subcommand, arguments)
    }

    #[allow(dead_code)]
    pub fn version(&self) -> Result<FlutterVersion, BuildError> {
        let mut cmd = self.base_command()?;
        cmd.arg("--version").arg("--machine");
        let output = cmd
            .output()
            .map_err(|e| BuildError::Io(format!("Failed to read flutter version: {}", e)))?;
        if !output.status.success() {
            return Err(BuildError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        let parsed: Value = serde_json::from_slice(&output.stdout).map_err(|e| {
            BuildError::Parse(format!(
                "Failed to parse flutter --version --machine JSON: {}",
                e
            ))
        })?;
        let flutter_version = parsed
            .get("flutterVersion")
            .or_else(|| parsed.get("frameworkVersion"))
            .and_then(Value::as_str)
            .map(ToString::to_string);
        Ok(FlutterVersion { flutter_version })
    }

    fn base_command(&self) -> Result<Command, BuildError> {
        let executable = self.resolve_executable()?;
        let mut cmd = Command::new(executable);
        if let Some(env) = self.environment {
            cmd.envs(env);
        }
        Ok(cmd)
    }

    fn resolve_executable(&self) -> Result<String, BuildError> {
        if let Some(env) = self.environment
            && let Some(root) = env.get("FLUTTER_ROOT")
            && !root.is_empty()
        {
            let path = PathBuf::from(root).join("bin").join("flutter");
            if !path.exists() {
                return Err(BuildError::Io(format!(
                    "FLUTTER_ROOT environment variable is set to a path that does not exist: {}",
                    root
                )));
            }
            return Ok(path.to_string_lossy().to_string());
        }
        Ok("flutter".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn uses_flutter_root_when_set() {
        let dir = tempdir().expect("tempdir");
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).expect("mkdir");
        let flutter = bin_dir.join("flutter");
        fs::write(&flutter, "#!/bin/sh\nexit 0\n").expect("write script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perm = fs::metadata(&flutter).expect("meta").permissions();
            perm.set_mode(0o755);
            fs::set_permissions(&flutter, perm).expect("chmod");
        }

        let mut env = HashMap::new();
        env.insert(
            "FLUTTER_ROOT".to_string(),
            dir.path().to_string_lossy().to_string(),
        );
        let command = FlutterCommand::new(Some(&env));
        let resolved = command.resolve_executable().expect("resolve executable");
        assert!(resolved.ends_with("bin/flutter"));
    }

    #[test]
    fn fails_when_flutter_root_does_not_exist() {
        let mut env = HashMap::new();
        env.insert(
            "FLUTTER_ROOT".to_string(),
            "/path/that/does/not/exist".to_string(),
        );
        let command = FlutterCommand::new(Some(&env));
        let err = command.resolve_executable().expect_err("must fail");
        assert!(
            err.to_string()
                .contains("FLUTTER_ROOT environment variable is set to a path that does not exist")
        );
    }
}
