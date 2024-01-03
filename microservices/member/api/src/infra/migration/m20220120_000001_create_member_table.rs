use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Member::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Member::UserId).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Member::UserName).string().not_null())
                    .col(ColumnDef::new(Member::Status).string().not_null())
                    .col(ColumnDef::new(Member::MemberType).string().not_null())
                    .col(ColumnDef::new(Member::CreditScore).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Member::Point).big_unsigned().not_null())
                    .col(ColumnDef::new(Member::Level).integer().not_null())
                    .col(ColumnDef::new(Member::Active).boolean().not_null())
                    .col(ColumnDef::new(Member::Description).string().not_null())
                    .col(ColumnDef::new(Member::Creator).uuid().null())
                    .col(ColumnDef::new(Member::Modifier).uuid().null())
                    .col(ColumnDef::new(Member::CheckSum).string().null())
                    .col(ColumnDef::new(Member::Region).string().null())
                    .col(ColumnDef::new(Member::GroupId).string().null())
                    .col(ColumnDef::new(Member::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Member::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Member::Enabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(Member::Version).integer().not_null().default(0))
                    .col(ColumnDef::new(Member::Deleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Member::DeletedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
    UserId,
    UserName,
    Status,
    MemberType,
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
