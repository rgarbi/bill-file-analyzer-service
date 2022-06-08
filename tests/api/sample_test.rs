use bill_file_analyzer_service::domain::sample_model::Sample;
use uuid::Uuid;

use crate::helper::spawn_app;

#[tokio::test]
async fn post_sample_works() {
    let app = spawn_app().await;

    let sample = Sample {
        id: Uuid::new_v4(),
        string: Uuid::new_v4().to_string(),
        number: 123123123,
        small_number: 12,
    };

    let response = app.post_sample(sample.to_json()).await;

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn post_sample_file_works() {
    let app = spawn_app().await;

    let sample = Sample {
        id: Uuid::new_v4(),
        string: Uuid::new_v4().to_string(),
        number: 123123123,
        small_number: 12,
    };

    let body = Vec::from(sample.to_json().as_bytes());
    let response = app.post_file_sample(body).await;

    // Assert
    assert!(response.status().is_success());
}
