use chrono::{DateTime, Utc};
use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serial_test::serial;
use uuid::Uuid;

use crate::setup::setup_test;

mod setup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub version: u32,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image_url: Option<String>,
}

#[tokio::test]
#[serial]
async fn insert_user() {
    let (client, url, _) = setup_test().await;

    let dto = CreateUserDto {
        email: "some@email.com",
        username: "user12345",
        password: "secure:12345678",
    };

    let req = client
        .post(url.join("/api/user").unwrap())
        .json(&dto)
        .build()
        .unwrap();

    let res = client.execute(req).await.unwrap();

    assert_eq!(
        res.status(),
        StatusCode::CREATED,
        "invalid created user status code"
    );

    let user: UserResponse = res.json().await.unwrap();

    assert_eq!(user.email, dto.email);
    assert_eq!(user.username, dto.username);
    assert_eq!(user.bio, None);
    assert_eq!(user.image_url, None);
}
