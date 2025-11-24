use super::todo_entity;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todo_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub todo_id: i64,
    pub content: String,
    pub status: String,
    pub altered_content: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub modified_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Todo,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Todo => Entity::belongs_to(todo_entity::Entity)
                .from(Column::TodoId)
                .to(todo_entity::Column::Id)
                .into(),
        }
    }
}

impl Related<todo_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
