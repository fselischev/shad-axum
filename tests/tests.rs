use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use shad_axum::app;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn root() {
    let app = app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}

#[tokio::test]
async fn create_user() {
    let mut app = app().into_service();
    let srv = ServiceExt::<Request<Body>>::ready(&mut app).await.unwrap();

    // so called table-driven testing
    let test_cases = vec![(
        srv.call(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "id": 1234,
                        "username": "rustacean"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap(),
        json!({
            "id": 1234,
            "username": "rustacean"
        }),
        srv.call(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "id": 5678,
                        "username": "👩👨👧"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap(),
        json!({
            "id": 5678,
            "username": "👩👨👧"
        }),
    )];

    // let test_cases = vec![(
    //     app.clone()
    //         .oneshot(
    //             Request::builder()
    //                 .method(http::Method::POST)
    //                 .uri("/users")
    //                 .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //                 .body(Body::from(
    //                     serde_json::to_vec(&json!({
    //                         "username": "rustacean"
    //                     }))
    //                     .unwrap(),
    //                 ))
    //                 .unwrap(),
    //         )
    //         .await
    //         .unwrap(),
    //     json!({
    //         "id": 1337,
    //         "username": "rustacean"
    //     }),
    //     app.oneshot(
    //         Request::builder()
    //             .method(http::Method::POST)
    //             .uri("/users")
    //             .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //             .body(Body::from(
    //                 serde_json::to_vec(&json!({
    //                     "username": "👩👨👧"
    //                 }))
    //                 .unwrap(),
    //             ))
    //             .unwrap(),
    //     )
    //     .await
    //     .unwrap(),
    //     json!({
    //         "id": 1337,
    //         "username": "👩👨👧"
    //     }),
    // )];

    for tc in test_cases {
        let (response, want) = (tc.0, tc.1);
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, want);
    }
}
