use shared::scylladb::ScyllaPool;

pub struct Migrator;

impl Migrator {
    pub async fn migrations() -> Result<(), Box<dyn std::error::Error>> {
        let session = ScyllaPool::connection().await;
        let keyspace = ScyllaPool::init_keyspace(session, "eventflow", 1).await?;

        ScyllaPool::init_table(session, &keyspace, "transaction", "
            id UUID PRIMARY KEY,
            transaction_type TEXT,
            status TEXT,
            user_id UUID,
            data TEXT,
            event_ids TEXT,
            rollback_id UUID,
            description TEXT,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            enabled BOOLEAN,
            version INT,
            deleted BOOLEAN,
            deleted_at TIMESTAMP
        ").await?;

        Ok(())
    }
}

#[tokio::test]
async fn cql_time_type() -> Result<(), Box<dyn std::error::Error>> {
    Migrator::migrations().await?;

    Ok(())
}