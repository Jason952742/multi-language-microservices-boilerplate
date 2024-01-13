use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Accounts::UserId).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Accounts::AccountName).string().not_null())
                    .col(ColumnDef::new(Accounts::Status).string().not_null())
                    .col(ColumnDef::new(Accounts::AccountType).string().not_null())
                    .col(ColumnDef::new(Accounts::CcyType).string().not_null())
                    .col(ColumnDef::new(Accounts::DepositCount).integer().not_null())
                    .col(ColumnDef::new(Accounts::TotalDeposit).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::WithdrawCount).integer().not_null())
                    .col(ColumnDef::new(Accounts::TotalWithdraw).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::EarnCount).integer().not_null())
                    .col(ColumnDef::new(Accounts::TotalEarn).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::SpendCount).integer().not_null())
                    .col(ColumnDef::new(Accounts::TotalSpend).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::CommissionCount).integer().not_null())
                    .col(ColumnDef::new(Accounts::TotalCommission).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::FrozenAmount).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::Balance).decimal_len(20, 8).not_null())
                    .col(ColumnDef::new(Accounts::Description).string().not_null())
                    .col(ColumnDef::new(Accounts::Creator).uuid().null())
                    .col(ColumnDef::new(Accounts::Modifier).uuid().null())
                    .col(ColumnDef::new(Accounts::CheckSum).string().null())
                    .col(ColumnDef::new(Accounts::Region).string().null())
                    .col(ColumnDef::new(Accounts::GroupId).string().null())
                    .col(ColumnDef::new(Accounts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Accounts::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Accounts::Enabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(Accounts::Version).integer().not_null().default(0))
                    .col(ColumnDef::new(Accounts::Deleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Accounts::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
    UserId,
    AccountName,
    Status,
    AccountType,
    CcyType,
    DepositCount,
    TotalDeposit,
    WithdrawCount,
    TotalWithdraw,
    EarnCount,
    TotalEarn,
    SpendCount,
    TotalSpend,
    CommissionCount,
    TotalCommission,
    FrozenAmount,
    Balance,
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
