use defaults::Defaults;

const DEFAULT_USER_NAME: &str = "ubuntu";

#[derive(Debug, Defaults, Clone)]
pub struct InstanceConfig {
    #[def = "DEFAULT_USER_NAME.to_string()"]
    pub(crate) user_name: String,
    pub(crate) key_path: Option<String>,
    pub(crate) instance_id: String,
}

#[derive(Debug, Defaults, Clone)]
pub struct AutoscalingGroupConfig {
    #[def = "DEFAULT_USER_NAME.to_string()"]
    pub(crate) user_name: String,
    pub(crate) key_path: Option<String>,
    pub(crate) autoscaling_group_id: String,
    pub(crate) instance_index: usize,
}

#[derive(Debug, Clone)]
pub enum EzsshConfig {
    Instance(InstanceConfig),
    AutoscalingGroup(AutoscalingGroupConfig),
}

impl EzsshConfig {
    pub fn get() -> EzsshConfig {
        EzsshConfig::Instance(InstanceConfig::default())
    }
}
