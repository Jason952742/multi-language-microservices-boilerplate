use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Members::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Members::UserId).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Members::UserName).string().not_null())
                    .col(ColumnDef::new(Members::Status).string().not_null())
                    .col(ColumnDef::new(Members::MemberType).string().not_null())
                    .col(ColumnDef::new(Members::SubEndDate).timestamp().not_null())
                    .col(ColumnDef::new(Members::CreditScore).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Members::Point).integer().not_null())
                    .col(ColumnDef::new(Members::Level).integer().not_null())
                    .col(ColumnDef::new(Members::Active).boolean().not_null())
                    .col(ColumnDef::new(Members::Description).string().not_null())
                    .col(ColumnDef::new(Members::Creator).uuid().null())
                    .col(ColumnDef::new(Members::Modifier).uuid().null())
                    .col(ColumnDef::new(Members::CheckSum).string().null())
                    .col(ColumnDef::new(Members::Region).string().null())
                    .col(ColumnDef::new(Members::GroupId).string().null())
                    .col(ColumnDef::new(Members::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Members::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Members::Enabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(Members::Version).integer().not_null().default(0))
                    .col(ColumnDef::new(Members::Deleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Members::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Members {
    Table,
    Id,
    UserId,
    UserName,
    Status,
    MemberType,
    SubEndDate,
    CreditScore,
    Point,
    Level,
    Active,
    Description,

    Creator,
    Modifier,
    CheckSum,
    Region,
    GroupId,
    CreatedAt,
    UpdatedAt,
    Enabled,
    Version,
    Deleted,
    DeletedAt,
}
