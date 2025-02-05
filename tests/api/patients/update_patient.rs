use crate::helpers::TestApplication;
use patient_api::PatientResponse;
use uuid::Uuid;

#[tokio::test]
async fn update_patient_when_invalid_birth_date_returns_bad_request() {
    //Arrange
    let app = TestApplication::run_app().await;
    let id = Uuid::new_v4();
    let patient_request_body = serde_json::json!({
          "name": {
            "family": "Zanko",
            "given": [
              "Raman"
            ]
          },
          "gender": "Male",
          "birth_date": "2024-01",
          "active": true,
          "version": 1
    });

    //Act
    let response = app.update_patient(&id, &patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.text().await.unwrap(), "invalid birth date format");
}

#[tokio::test]
async fn update_patient_when_patient_does_not_exist_returns_404() {
    //Arrange
    let app = TestApplication::run_app().await;
    let id = Uuid::new_v4();
    let patient_request_body = serde_json::json!({
          "name": {
            "family": "Zanko",
            "given": [
              "Raman"
            ]
          },
          "gender": "Male",
          "birth_date": "2024-01-01",
          "active": true,
          "version": 1
    });

    //Act
    let response = app.update_patient(&id, &patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 404);
    assert!(response.text().await.unwrap().contains("Patient not found"));
}

#[tokio::test]
async fn update_patient_when_invalid_version_returns_409() {
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

    let patient_request_body = serde_json::json!({
          "name": {
            "family": "Zanko",
            "given": [
              "Raman"
            ]
          },
          "gender": "Male",
          "birth_date": "2024-01-01",
          "active": true,
          "version": 1
    });

    //Act
    let response = app.update_patient(&id, &patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 409);
    assert!(response
        .text()
        .await
        .unwrap()
        .contains("Patient was changed"));
}

#[tokio::test]
async fn update_patient_when_valid_returns_204() {
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
    let version = patient_response.version;

    let patient_request_body = serde_json::json!({
          "name": {
            "family": "Test",
            "given": [
              "Test"
            ]
          },
          "gender": "Female",
          "birth_date": "2020-01-01",
          "active": false,
          "version": version
    });

    //Act
    let response = app.update_patient(&id, &patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 204);

    let response = app.get_patient_by_id(&id).await;
    let patient_response = response.json::<PatientResponse>().await.unwrap();

    assert_ne!(version, patient_response.version);
}
