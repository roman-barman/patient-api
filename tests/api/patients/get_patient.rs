use crate::helpers::run_app;

#[tokio::test]
async fn get_patient_works() {
    //Arrange
    run_app(8083).await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http:/127.0.0.1:8083/patients/12345")
        .send()
        .await
        .expect("Could not send request to server");

    //Assert
    assert!(response.status().is_success());
}
