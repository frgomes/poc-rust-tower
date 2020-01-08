#[macro_use] extern crate serde;
#[macro_use] extern crate serde_plain;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_yaml;
#[macro_use] extern crate futures;

extern crate poc_rust_tower;

mod apis {
    mod table {
        async fn async_table_update(data: poc_rust_tower::models::TableUpdateRequest) ->
            Result<poc_rust_tower::models::TableUpdateResponse, Box<dyn std::error::Error + Send + Sync>> {
                /*
                use isahc::prelude::*;
                use std::time::Duration;
                // let request: Request<poc_rust_tower::models::TableUpdateRequest> =
                let request =
                    Request::post("http://127.0.0.1:8080/api/table/update")
                    .header("Content-Type", "application/json")
                    .timeout(Duration::from_secs(5))
                    .body(serde_json::to_string(&data)?)?;
                let mut response = request.send()?;
                let json = response.json::<poc_rust_tower::models::TableUpdateResponse>()?;
                Ok(json)
                */
                let request: hyper::Request<poc_rust_tower::models::TableUpdateRequest> =
                    hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri("http://127.0.0.1:8080/api/table/update")
                    .header(hyper::header::CONTENT_TYPE, "application/json")
                    .body(data.into())
                    .unwrap();
                
                    //.unwrap();
                let client = hyper::Client::new();
                let response = client.request(request); // .await?;
                //XXX let body = hyper::body::aggregate(response).await?;
                let actual: poc_rust_tower::models::TableUpdateResponse = serde_json::from(body)?;
                actual
            }

        #[test]
        fn table_update() {
            let request =
                poc_rust_tower::models::TableUpdateRequest {
                    version: "1.0.0".to_owned(),
                    table: poc_rust_tower::models::DataTable {
                        name: "table".to_owned(),
                        cells: vec! {
                            poc_rust_tower::models::DataCell {
                                coord: poc_rust_tower::models::Coord { row: 1, col: 2 },
                                value: "Bart Simpson".to_owned(),
                            },
                        },
                    },
                };
            let expected =
                poc_rust_tower::models::TableUpdateResponse {
                    version: "1.0.0".to_owned(),
                    table: poc_rust_tower::models::ErrorTable {
                        name: "table".to_owned(),
                        cells: vec! {
                            poc_rust_tower::models::ErrorCell {
                                coord: poc_rust_tower::models::Coord { row: 1, col: 2 },
                                error: poc_rust_tower::models::Error { code: 10123, message: "XXX a number is expected".to_owned() },
                            },
                        },
                    },
                };

            use tokio::io;
            use tokio::net;
            use tokio::prelude::*;

            //XXX let operation: std::future::Future<io::error::Result<poc_rust_tower::models::TableUpdateResponse, Box<dyn std::error::Error + Send + Sync>>> = async_table_update(request);
            let operation = async_table_update(request);
            let future = operation
                .and_then(|result|
                          match result {
                              Ok(actual) =>
                                  if(actual == expected) {
                                      Ok("Request succeeded but returned unexpected results")
                                  } else {
                                      Err("Request succeeded but returned unexpected results") },
                              Err(e) => Err(e)
                          })
                .map(|_| println!("Request succeeded."))
                .map_err(|_| { println!("Request failed."); assert!(false) });

            tokio::run(future);
        }
    }
}
