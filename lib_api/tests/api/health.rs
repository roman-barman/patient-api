use crate::helpers::TestApplication;

#[tokio::test]
async fn health_check_returns_200() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.health_check().await;

    //Assert
    assert_eq!(response.status().as_u16(), 200);
}
