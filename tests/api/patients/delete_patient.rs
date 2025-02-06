use crate::helpers::TestApplication;
use patient_api::PatientResponse;
use uuid::Uuid;

#[tokio::test]
async fn delete_patient_when_patient_not_exist_404() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.delete_patient(&Uuid::new_v4()).await;

    //Assert
    assert_eq!(response.status().as_u16(), 404);
    assert!(response.text().await.unwrap().contains("Patient not found"));
}

#[tokio::test]
async fn delete_patient_when_patient_exist_returns_204() {
    //Arrange
    let app = TestApplication::run_app().await;
    let patient_request_body = serde_json::json!({
          "name": {
            "family": "Zanko",
            "given": [
              "Raman"
            ]
          },
          "gender": "Male",
          "birth_date": "2024-01-01",
          "active": true
    });
    let response = app.create_patient(&patient_request_body).await;
    let patient_response = response.json::<PatientResponse>().await.unwrap();
    let id = patient_response.name.id;

    //Act
    let response = app.delete_patient(&id).await;

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 204);

    let response = app.get_patient_by_id(&id).await;
    assert_eq!(response.status().as_u16(), 404);
}
