use rand::prelude::ThreadRng;
use rand::Rng;
use random_string::generate;
use std::env;

const CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm-";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let variable = "APP_API_ADDRESS";
    let env = env::var(variable)?;

    let client = reqwest::Client::new();

    println!("Start feeder");

    let mut rng = rand::rng();

    for _ in 0..100 {
        let gender = generate_gender(&mut rng);
        let is_active = generate_is_active(&mut rng);
        let birth_date = generate_birth_date(&mut rng);
        let family = generate_family();
        let given = generate_given(&mut rng);

        let body = serde_json::json!({
              "name": {
                "family": family,
                "given":given
              },
              "gender": gender,
              "birth_date": birth_date,
              "active": is_active
        });

        let response = client
            .post(format!("{}/patients", &env))
            .json(&body)
            .send()
            .await
            .expect("Could not send request to server");

        if response.status() != reqwest::StatusCode::CREATED {
            return Err(anyhow::anyhow!(response.text().await?));
        }
    }

    println!("End feeder");

    Ok(())
}

fn generate_gender(rng: &mut ThreadRng) -> String {
    let number = rng.random_range(1..=2);

    if number == 1 {
        "Male".to_string()
    } else {
        "Female".to_string()
    }
}

fn generate_is_active(rng: &mut ThreadRng) -> bool {
    rng.random_bool(1.0 / 3.0)
}

fn generate_birth_date(rng: &mut ThreadRng) -> String {
    let year = rng.random_range(1990..=2020);
    format!("{}-01-01", year)
}

fn generate_family() -> String {
    generate(10, CHARSET)
}

fn generate_given(rng: &mut ThreadRng) -> Vec<String> {
    let count = rng.random_range(1..=5);
    (0..count).map(|_| generate(6, CHARSET)).collect()
}
