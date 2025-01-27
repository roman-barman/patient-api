use crate::helpers::TestApplication;

#[tokio::test]
async fn get_all_patients_works() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.get_all_patients().await;

    //Assert
    assert!(response.status().is_success());
}
