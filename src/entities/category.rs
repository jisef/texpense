//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "texpense", table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_category: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub pre_category: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::PreCategory",
        to = "Column::IdCategory",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
