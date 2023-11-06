use crate::command::EzsshCommand;
use crate::config::{AutoscalingGroupConfig, InstanceConfig};
use crate::error::EzsshError::CommandParseError;
use crate::error::Result;

pub(crate) async fn command_from_instance_config(config: &InstanceConfig) -> Result<EzsshCommand> {
    let aws_cfg = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&aws_cfg);
    let ip = get_ip_from_instance_id(&client, &config.instance_id).await?;

    Ok(EzsshCommand {
        host: ip,
        user_name: config.user_name.clone(),
        ssh_key_path: config.key_path.clone(),
    })
}

async fn get_ip_from_instance_id(
    client: &aws_sdk_ec2::Client,
    instance_id: &String,
) -> Result<String> {
    let describe_instances_output = client
        .describe_instances()
        .set_instance_ids(vec![instance_id.clone()].into())
        .send()
        .await?;
    // expecting 1 and only 1 instance in describe_instances_output
    let reservations = describe_instances_output.reservations();
    if reservations.len() != 1 {
        return Err(CommandParseError("wrong number reservations".to_string()));
    }
    let reservation = &reservations[0];
    let instances = reservation.instances();
    if instances.len() != 1 {
        return Err(CommandParseError("wrong number of instances".to_string()));
    }
    let instance = &instances[0];
    if let Some(response_instance_id) = instance.instance_id() {
        if response_instance_id == instance_id.as_str() {
            return Ok(instance.public_ip_address().unwrap_or_default().to_string());
        }
    }
    Err(CommandParseError(format!(
        "No public ip found for instance {instance_id}"
    )))
}

pub(crate) async fn command_from_asg_config(
    config: &AutoscalingGroupConfig,
) -> Result<EzsshCommand> {
    let aws_cfg = aws_config::load_from_env().await;
    let autoscaling_client = aws_sdk_autoscaling::client::Client::new(&aws_cfg);
    let autoscaling_group_result = autoscaling_client
        .describe_auto_scaling_groups()
        .set_auto_scaling_group_names(vec![config.autoscaling_group_id.clone()].into())
        .send()
        .await?;
    let groups = autoscaling_group_result.auto_scaling_groups();
    if groups.len() != 1 {
        return Err(CommandParseError("Wrong number of groups".to_string()));
    }
    let group = &groups[0];
    let instances = group.instances();
    if instances.is_empty() {
        return Err(CommandParseError("Empty Autoscaling group".to_string()));
    }
    let mut instance_ids = Vec::new();
    for instance in group.instances() {
        if let Some(instance_id) = &instance.instance_id {
            instance_ids.push(instance_id);
        }
    }
    // deterministic order of instance id's
    instance_ids.sort();
    let instance_id = instance_ids.get(config.instance_index).ok_or(CommandParseError(format!(
        "Invalid requested instance index, there are currently {} instances in autoscaling group, valid indexes are in the range [0, {}].",
        instance_ids.len(),
        instance_ids.len() - 1)
    ))?;

    let ec2_client = aws_sdk_ec2::Client::new(&aws_cfg);
    let ip = get_ip_from_instance_id(&ec2_client, instance_id).await?;

    Ok(EzsshCommand {
        host: ip,
        user_name: config.user_name.clone(),
        ssh_key_path: config.key_path.clone(),
    })
}
