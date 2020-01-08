#[derive(Clone, Debug)]
pub struct Endpoints;

impl_web! {
    impl Endpoints {
        #[post("/table/update")]
        fn table_update(&self, body: crate::models::TableUpdateRequest) -> Result<crate::models::TableUpdateResponse, crate::models::Error> {
            Ok(crate::models::TableUpdateResponse {
                version: "1.0.0".to_owned(),
                table: crate::models::ErrorTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::ErrorCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            error: crate::models::Error { code: 10123, message: "a number is expected".to_owned() },
                        },
                    },
                },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn table_update() {
        let endpoints = crate::apis::table::Endpoints;
        let request =
            crate::models::TableUpdateRequest {
                version: "1.0.0".to_owned(),
                table: crate::models::DataTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::DataCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            value: "Bart Simpson".to_owned(),
                        },
                    },
                },
            };
        let response =
            crate::models::TableUpdateResponse {
                version: "1.0.0".to_owned(),
                table: crate::models::ErrorTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::ErrorCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            error: crate::models::Error { code: 10123, message: "a number is expected".to_owned() },
                        },
                    },
                },
            };
        assert_eq!(endpoints.table_update(request), Ok(response));
    }
}   
