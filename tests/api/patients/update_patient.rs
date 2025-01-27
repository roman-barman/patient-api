use crate::helpers::TestApplication;

#[tokio::test]
async fn update_patient_works() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.update_patient("12345").await;

    //Assert
    assert!(response.status().is_success());
}
