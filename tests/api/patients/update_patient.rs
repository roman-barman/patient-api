use crate::helpers::run_app;

#[tokio::test]
async fn update_patient_works() {
    //Arrange
    run_app(8084).await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .put("http:/127.0.0.1:8084/patients/12345")
        .send()
        .await
        .expect("Could not send request to server");

    //Assert
    assert!(response.status().is_success());
}
