// @generated automatically by Diesel CLI.

diesel::table! {
    containers (container_id) {
        container_id -> Integer,
        #[max_length = 150]
        container_name -> Nullable<Varchar>,
        #[max_length = 150]
        container_image -> Nullable<Varchar>,
        #[max_length = 100]
        container_state -> Nullable<Varchar>,
        #[max_length = 100]
        container_status -> Nullable<Varchar>,
        service_id -> Nullable<Integer>,
    }
}

diesel::table! {
    servers (server_id) {
        server_id -> Integer,
        #[max_length = 30]
        server_name -> Varchar,
        #[max_length = 20]
        server_address -> Varchar,
        #[max_length = 30]
        server_username -> Varchar,
        #[max_length = 50]
        server_password -> Varchar,
        server_available -> Nullable<Bool>,
    }
}

diesel::table! {
    services (service_id) {
        service_id -> Integer,
        #[max_length = 50]
        service_name -> Varchar,
        service_port -> Integer,
        #[max_length = 30]
        service_username -> Nullable<Varchar>,
        #[max_length = 50]
        service_password -> Nullable<Varchar>,
        service_available -> Nullable<Bool>,
        server_id -> Nullable<Integer>,
    }
}

diesel::joinable!(containers -> services (service_id));
diesel::joinable!(services -> servers (server_id));

diesel::allow_tables_to_appear_in_same_query!(
    containers,
    servers,
    services,
);
