use crate::helpers::TestApplication;

#[tokio::test]
async fn get_patient_works() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.get_patient_by_id("12345").await;

    //Assert
    assert!(response.status().is_success());
}
