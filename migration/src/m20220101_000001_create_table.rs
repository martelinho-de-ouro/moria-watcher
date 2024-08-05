use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Guest::Id)
                            .uuid()
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Guest::Name).string().not_null())
                    .col(
                        ColumnDef::new(Guest::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Guest::Phone).string().not_null())
                    .col(ColumnDef::new(Guest::Address).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guest {
    Table,
    Id,
    Name,
    Email,
    Phone,
    Address,
}
