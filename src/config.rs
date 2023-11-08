use clap::{Args, Parser};
use defaults::Defaults;

const DEFAULT_USER_NAME: &str = "ubuntu";

#[derive(Debug, Defaults, Clone, Args)]
pub struct InstanceConfig {
    // #[def = "DEFAULT_USER_NAME.to_string()"]
    #[clap(short = 'u', long = "username", default_value = DEFAULT_USER_NAME)]
    pub(crate) user_name: String,
    #[clap(short = 'k', long = "key")]
    pub(crate) key_path: Option<String>,
    #[clap(short = 'i', long = "instance_id")]
    pub(crate) instance_id: String,
}

#[derive(Debug, Defaults, Clone, Args)]
pub struct AutoscalingGroupConfig {
    // #[def = "DEFAULT_USER_NAME.to_string()"]
    #[clap(short = 'u', long = "username", default_value = DEFAULT_USER_NAME)]
    pub(crate) user_name: String,
    #[clap(short = 'k', long = "key")]
    pub(crate) key_path: Option<String>,
    #[clap(short = 'g', long = "asg")]
    pub(crate) autoscaling_group_id: String,
    #[clap(short = 'i', long = "instance", default_value = "0")]
    pub(crate) instance_index: usize,
}

#[derive(Debug, Clone, Parser)]
pub enum EzsshConfig {
    Instance(InstanceConfig),
    ASG(AutoscalingGroupConfig),
}

impl EzsshConfig {
    pub fn get() -> EzsshConfig {
        EzsshConfig::parse()
    }
}
