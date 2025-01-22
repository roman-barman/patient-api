use crate::helpers::run_app;

#[tokio::test]
async fn get_all_patients_works() {
    //Arrange
    run_app(8082).await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http:/127.0.0.1:8082/patients")
        .send()
        .await
        .expect("Could not send request to server");

    //Assert
    assert!(response.status().is_success());
}
