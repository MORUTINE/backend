use sea_orm::entity::prelude::*;
use crate::database::postgres::user::social_account_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub nickname: String,
    pub profile_image_url: Option<String>,
    pub created_at: DateTimeUtc,
    pub modified_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SocialAccounts,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::SocialAccounts => Entity::has_many(social_account_entity::Entity).into(),
        }
    }
}

impl Related<social_account_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SocialAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
