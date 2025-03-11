//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "texpense", table_name = "payment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id_payment: i32,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    pub category: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub label: String,
    pub templatefk: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::Category",
        to = "super::category::Column::IdCategory",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::template::Entity",
        from = "Column::Templatefk",
        to = "super::template::Column::IdTemplate",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Template,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Template.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
