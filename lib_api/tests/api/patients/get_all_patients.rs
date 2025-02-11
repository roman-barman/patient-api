use crate::helpers::TestApplication;
use lib_api::PatientResponse;

#[tokio::test]
async fn get_all_patients_when_no_patients_returns_200() {
    //Arrange
    let app = TestApplication::run_app().await;

    //Act
    let response = app.get_all_patients().await;

    //Assert
    assert_eq!(response.status().as_u16(), 200);

    let patients = response.json::<Vec<PatientResponse>>().await.unwrap();
    assert!(patients.is_empty());
}

#[tokio::test]
async fn get_all_patients_returns_200() {
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
    let _ = app.create_patient(&patient_request_body).await;
    let _ = app.create_patient(&patient_request_body).await;

    //Act
    let response = app.get_all_patients().await;

    //Assert
    assert_eq!(response.status().as_u16(), 200);

    let patients = response.json::<Vec<PatientResponse>>().await.unwrap();
    assert_eq!(patients.len(), 2);
}
