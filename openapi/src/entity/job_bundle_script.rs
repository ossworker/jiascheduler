//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "job_bundle_script")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub eid: String,
    pub executor_id: u64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub code: String,
    pub info: String,
    pub created_user: String,
    pub updated_user: String,
    pub args: Option<Json>,
    pub created_time: DateTimeUtc,
    pub updated_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
