//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "texpense", table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_transaction: i32,
    pub src_account: Option<i32>,
    pub dest_account: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::DestAccount",
        to = "super::account::Column::IdAccount",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Account2,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::SrcAccount",
        to = "super::account::Column::IdAccount",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Account1,
}

impl ActiveModelBehavior for ActiveModel {}
