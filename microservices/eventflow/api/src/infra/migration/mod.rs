use shared::datasource::scylladb::ScyllaPool;
use crate::domain::aggregates::account_ar::Account;
use crate::domain::aggregates::member_ar::Member;
use crate::domain::aggregates::referral_ar::Referral;

pub struct Migrator;

impl Migrator {
    pub async fn migrations() -> Result<(), Box<dyn std::error::Error>> {
        let session = ScyllaPool::connection().await;
        let keyspace = ScyllaPool::init_keyspace(session, "eventflow", 1).await?;

        ScyllaPool::init_table(session, &keyspace, "transactions", "
            id UUID,
            transaction_type TEXT,
            status TEXT,
            user_id UUID,
            payload TEXT,
            events TEXT,
            rollback_id UUID,
            description TEXT,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            enabled BOOLEAN,
            version INT,
            deleted BOOLEAN,
            deleted_at TIMESTAMP,
        ", "PRIMARY KEY (id)", "").await?;

        ScyllaPool::init_table(session, &keyspace, Account::TABLE_NAME, "
            id UUID,
            txn_id UUID,
            aggregate_id UUID,
            aggregate_type TEXT,
            sequence BIGINT,
            event_type TEXT,
            event_version TEXT,
            payload TEXT,
            metadata TEXT,
            created_at TIMESTAMP,
        ", "PRIMARY KEY (aggregate_id, sequence)", "WITH CLUSTERING ORDER BY (sequence DESC)").await?;

        ScyllaPool::init_table(session, &keyspace, Member::TABLE_NAME, "
            id UUID,
            txn_id UUID,
            aggregate_id UUID,
            aggregate_type TEXT,
            sequence BIGINT,
            event_type TEXT,
            event_version TEXT,
            payload TEXT,
            metadata TEXT,
            created_at TIMESTAMP,
        ", "PRIMARY KEY (aggregate_id, sequence)", "WITH CLUSTERING ORDER BY (sequence DESC)").await?;

        ScyllaPool::init_table(session, &keyspace, Referral::TABLE_NAME, "
            id UUID,
            txn_id UUID,
            aggregate_id UUID,
            aggregate_type TEXT,
            sequence BIGINT,
            event_type TEXT,
            event_version TEXT,
            payload TEXT,
            metadata TEXT,
            created_at TIMESTAMP,
        ", "PRIMARY KEY (aggregate_id, sequence)", "WITH CLUSTERING ORDER BY (sequence DESC)").await?;

        Ok(())
    }
}

#[tokio::test]
async fn cql_time_type() -> Result<(), Box<dyn std::error::Error>> {
    Migrator::migrations().await?;

    Ok(())
}