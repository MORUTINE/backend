use crate::database::postgres::user::user_entity;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "social_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub provider: String,
    pub provider_user_id: String,
    pub created_at: DateTimeUtc,
    pub modified_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Users => Entity::belongs_to(user_entity::Entity)
                .from(Column::UserId)
                .to(user_entity::Column::Id)
                .into(),
        }
    }
}

impl Related<user_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
