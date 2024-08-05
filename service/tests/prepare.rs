use ::entity::guest;
use sea_orm::*;
use uuid::Uuid;

#[cfg(feature = "mock")]
#[allow(clippy::unwrap_used)]
fn get_uuid(n: &str) -> Uuid {
    let uuid = Uuid::parse_str(n);
    uuid.unwrap()
}

pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([
            [guest::Model {
                id: get_uuid("b04965e6-a9bb-591f-8f8a-1adcb2c8dc39"),
                name: "name A".to_owned(),
                email: "email @ A".to_owned(),
                phone: "123 A".to_owned(),
                address: "AA A".to_owned(),
            }],
            [guest::Model {
                id: get_uuid("4b166dbe-d99d-5091-abdd-95b83330ed3a"),
                name: "name B".to_owned(),
                email: "email @ B".to_owned(),
                phone: "123 B".to_owned(),
                address: "AA B".to_owned(),
            }],
            [guest::Model {
                id: get_uuid("98123fde-012f-5ff3-8b50-881449dac91a"),
                name: "name C".to_owned(),
                email: "email @ C".to_owned(),
                phone: "123 C".to_owned(),
                address: "AA C".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 3,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 3,
                rows_affected: 3,
            },
        ])
        .into_connection()
}
