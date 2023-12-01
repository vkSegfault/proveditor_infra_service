// @generated automatically by Diesel CLI.

diesel::table! {
    _user (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    country (name) {
        #[max_length = 255]
        capital -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        color -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    factory (name) {
        price -> Int4,
        time_to_produce -> Int4,
        #[max_length = 255]
        input_resource -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        output_resource -> Nullable<Varchar>,
        input_resources_amount -> Nullable<Array<Nullable<Int4>>>,
        output_resources_amount -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    factory_input_resources (factory_name, input_resources_name) {
        #[max_length = 255]
        factory_name -> Varchar,
        #[max_length = 255]
        input_resources_name -> Varchar,
    }
}

diesel::table! {
    factory_output_resources (factory_name, output_resources_name) {
        #[max_length = 255]
        factory_name -> Varchar,
        #[max_length = 255]
        output_resources_name -> Varchar,
    }
}

diesel::table! {
    infra (name) {
        #[max_length = 255]
        name -> Varchar,
        infra_modifier -> Nullable<Float8>,
        price -> Nullable<Int4>,
    }
}

diesel::table! {
    province (name) {
        pop -> Nullable<Int4>,
        #[max_length = 1024]
        shape -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
        #[max_length = 255]
        factory -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        terrain -> Nullable<Varchar>,
        color -> Nullable<Array<Nullable<Int4>>>,
        resources_amounts -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    province_resources (province_name, resources_name) {
        #[max_length = 255]
        province_name -> Varchar,
        #[max_length = 255]
        resources_name -> Varchar,
    }
}

diesel::table! {
    resource (name) {
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        resources_name -> Nullable<Varchar>,
        #[max_length = 255]
        resources -> Nullable<Varchar>,
    }
}

diesel::table! {
    terrain (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(factory_input_resources -> factory (factory_name));
diesel::joinable!(factory_input_resources -> resource (input_resources_name));
diesel::joinable!(factory_output_resources -> factory (factory_name));
diesel::joinable!(factory_output_resources -> resource (output_resources_name));
diesel::joinable!(province -> country (country));
diesel::joinable!(province -> factory (factory));
diesel::joinable!(province -> terrain (terrain));
diesel::joinable!(province_resources -> province (province_name));
diesel::joinable!(province_resources -> resource (resources_name));

diesel::allow_tables_to_appear_in_same_query!(
    _user,
    country,
    factory,
    factory_input_resources,
    factory_output_resources,
    infra,
    province,
    province_resources,
    resource,
    terrain,
);
