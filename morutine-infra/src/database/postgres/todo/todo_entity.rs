use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub date: Date,
    pub created_at: DateTimeWithTimeZone,
    pub modified_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TodoItems,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TodoItems => Entity::has_many(super::todo_item_entity::Entity).into(),
        }
    }
}

impl Related<super::todo_item_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TodoItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
