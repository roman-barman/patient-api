use crate::helpers::TestApplication;
use lib_api::PatientResponse;

#[tokio::test]
async fn create_patient_when_valid_body_returns_created() {
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

    //Act
    let response = app.create_patient(&patient_request_body).await;

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 201);

    let location_header = response.headers().get("Location").unwrap().clone();
    let patient_response = response.json::<PatientResponse>().await.unwrap();

    assert_eq!(
        location_header.to_str().unwrap(),
        format!("/patients/{}", patient_response.name.id)
    );

    assert!(patient_response.version > 0);
}

#[tokio::test]
async fn create_patient_when_invalid_birth_date_returns_bad_request() {
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
          "birth_date": "2024-01",
          "active": true
    });

    //Act
    let response = app.create_patient(&patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.text().await.unwrap(), "invalid birth date format");
}

#[tokio::test]
async fn create_patient_when_invalid_family_returns_bad_request() {
    //Arrange
    let app = TestApplication::run_app().await;
    let patient_request_body = serde_json::json!({
          "name": {
            "family": "AsdfghjklqwertyuiopzxcvbnmAsdfghjklqwertyuiopzxcvbnmAsdfghjklqwertyuiopzxcvbnmAsdfghjklqwertyuiopzxcvbnm",
            "given": [
              "Raman"
            ]
          },
          "gender": "Male",
          "birth_date": "2024-01-01",
          "active": true
    });

    //Act
    let response = app.create_patient(&patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        response.text().await.unwrap(),
        "family length is greater than 100"
    );
}

#[tokio::test]
async fn create_patient_when_name_not_exist_returns_bad_request() {
    //Arrange
    let app = TestApplication::run_app().await;
    let patient_request_body = serde_json::json!({
          "gender": "Male",
          "birth_date": "2024-01-01",
          "active": true
    });

    //Act
    let response = app.create_patient(&patient_request_body).await;

    //Assert
    assert_eq!(response.status().as_u16(), 400);
    assert!(response
        .text()
        .await
        .unwrap()
        .contains("Json deserialize error: missing field `name`"));
}
