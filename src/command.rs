use crate::config::EzsshConfig;
use crate::error::EzsshError;
use std::os::unix::process::CommandExt;
use std::process::Command;

const SSH_BIN: &str = "ssh";

#[derive(Debug, Clone, Default)]
pub struct EzsshCommand {
    pub(crate) host: String,
    pub(crate) user_name: String,
    pub(crate) ssh_key_path: Option<String>,
}

impl EzsshCommand {
    pub async fn from_config(config: &EzsshConfig) -> Result<EzsshCommand, EzsshError> {
        match config {
            EzsshConfig::Instance(instance_config) => {
                crate::aws::command_from_instance_config(instance_config).await
            }
            EzsshConfig::ASG(asg_config) => crate::aws::command_from_asg_config(asg_config).await,
        }
    }

    pub fn exec(self) -> Result<(), EzsshError> {
        let mut command: Command = self.into();
        let err = command.exec();
        // if exec returns an err, we'll bubble it up to main
        Err(err.into())
    }
}

impl Into<Command> for EzsshCommand {
    fn into(self) -> Command {
        let mut command = Command::new(SSH_BIN);
        command.args(build_ssh_args(&self));
        command
    }
}

fn build_ssh_args(command: &EzsshCommand) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(key_path) = &command.ssh_key_path {
        args.push("-i".to_string());
        args.push(key_path.clone());
    }
    args.push(user_host_string(command));

    args
}

fn user_host_string(command: &EzsshCommand) -> String {
    let EzsshCommand {
        host, user_name, ..
    } = command;
    format!("{user_name}@{host}")
}
