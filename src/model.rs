use diesel::{query_builder::AsChangeset, prelude::Insertable, deserialize::Queryable, Selectable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, Identifiable, AsChangeset, IntoParams, ToSchema)]
#[diesel(primary_key(name))]
#[diesel(table_name = crate::schema::infra)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Infra {

    #[schema(example = "airport")]
    pub name: String,

    #[schema(example = 0.35)]
    pub infra_modifier: Option<f64>,

    #[schema(example = 2000)]
    pub price: Option<i32>
}