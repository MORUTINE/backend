use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. users 테이블
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Nickname).string().not_null())
                    .col(ColumnDef::new(Users::ProfileImageUrl).string().null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. social_accounts 테이블
        manager
            .create_table(
                Table::create()
                    .table(SocialAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SocialAccounts::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SocialAccounts::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SocialAccounts::Provider).string().not_null())
                    .col(
                        ColumnDef::new(SocialAccounts::ProviderUserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SocialAccounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SocialAccounts::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-social_accounts-user_id")
                            .from(SocialAccounts::Table, SocialAccounts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. todos 테이블
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todos::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Todos::Date).date().not_null())
                    .col(
                        ColumnDef::new(Todos::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Todos::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-todos-user_id")
                            .from(Todos::Table, Todos::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. todo_items 테이블
        manager
            .create_table(
                Table::create()
                    .table(TodoItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TodoItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoItems::TodoId).big_integer().not_null())
                    .col(ColumnDef::new(TodoItems::Content).string().not_null())
                    .col(ColumnDef::new(TodoItems::Status).string().not_null())
                    .col(ColumnDef::new(TodoItems::AlteredContent).string().null())
                    .col(ColumnDef::new(TodoItems::ImageUrl).string().null())
                    .col(
                        ColumnDef::new(TodoItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TodoItems::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-todo_items-todo_id")
                            .from(TodoItems::Table, TodoItems::TodoId)
                            .to(Todos::Table, Todos::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SocialAccounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Nickname,
    ProfileImageUrl,
    CreatedAt,
    ModifiedAt,
}

#[derive(Iden)]
enum SocialAccounts {
    Table,
    Id,
    UserId,
    Provider,
    ProviderUserId,
    CreatedAt,
    ModifiedAt,
}

#[derive(Iden)]
enum Todos {
    Table,
    Id,
    UserId,
    Date,
    CreatedAt,
    ModifiedAt,
}

#[derive(Iden)]
enum TodoItems {
    Table,
    Id,
    TodoId,
    Content,
    Status,
    AlteredContent,
    ImageUrl,
    CreatedAt,
    ModifiedAt,
}
