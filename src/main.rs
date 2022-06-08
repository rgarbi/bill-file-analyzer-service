use bill_file_analyzer_service::configuration::get_configuration;
use bill_file_analyzer_service::startup::Application;
use bill_file_analyzer_service::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "bill-file-analyzer-service".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
