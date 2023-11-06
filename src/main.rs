use ezssh::command::EzsshCommand;
use ezssh::config::EzsshConfig;
use ezssh::error::EzsshError;
use ezssh::error_printer::ErrorPrinter;

#[tokio::main]
async fn main() -> Result<(), EzsshError> {
    let config = EzsshConfig::get()?;

    let command = EzsshCommand::from_config(config).await?;

    command.exec().log_error("error from exec'ing ssh")
}
