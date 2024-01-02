use std::env;
use neo4rs::{ConfigBuilder, Graph};
use tokio::sync::OnceCell;
use tracing::info;
use colored::Colorize;

#[derive(Debug)]
pub struct Neo4jPool;

static CLIENT: OnceCell<Graph> = OnceCell::const_new();

impl Neo4jPool {
    pub async fn graph() -> &'static Graph {
        CLIENT
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let uri = env::var("NEO4J_URI").expect("NEO4J_URI must be set");
                let user = env::var("NEO4J_USER").expect("NEO4J_USER must be set");
                let password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD must be set");

                let config = ConfigBuilder::default()
                    .uri(uri)
                    .user(user)
                    .password(password)
                    .db("neo4j")
                    .fetch_size(500)
                    .max_connections(10)
                    .build()
                    .unwrap();
                let graph = Graph::connect(config).await.expect("Neo4J connection failed");
                info!("{}", "Neo4j CONNECTED".color("magenta"));
                graph
            })
            .await
    }
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use neo4rs::{query};

    let graph = Neo4jPool::graph().await;
    {
        let id = uuid::Uuid::new_v4().to_string();

        // graph
        //     .run(query("CREATE (p:Person {id: $id, age: 34, name: 'hello'})").param("id", id.clone()))
        //     .await.unwrap();

        graph.run(
            query("CREATE (p:Person {id: $id, age: 34, name: 'hello'})")
                .params([
                    ("id", id.clone())
                ])
        ).await.unwrap();

        let mut handles = Vec::new();
        let count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        for _ in 1..=42 {
            let id = id.clone();
            let count = count.clone();

            let handle = tokio::spawn(async move {
                let graph = Neo4jPool::graph().await;
                let mut result = graph
                    .execute(query("MATCH (p:Person {id: $id}) RETURN p").param("id", id))
                    .await.expect("Failed to execute");

                while let Ok(Some(_row)) = result.next().await {
                    count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        futures::future::join_all(handles).await;
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 42);
    }

    Ok(())
}
