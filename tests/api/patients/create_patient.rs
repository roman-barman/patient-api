use crate::helpers::run_app;

#[tokio::test]
async fn create_patient_works() {
    //Arrange
    run_app(8081).await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post("http:/127.0.0.1:8081/patients")
        .send()
        .await
        .expect("Could not send request to server");

    //Assert
    assert!(response.status().is_success());
}
