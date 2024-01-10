use shared::neo4rs::{self, query, Relation};
use uuid::Uuid;
use shared::datasource::neo4j::Neo4jPool;
use crate::domain::entities::member;

pub struct MemberDbMutation;

impl MemberDbMutation {
    pub async fn create_member(form_data: member::Model) -> Result<member::Model, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let _ = graph.run(
            query("CREATE (m:Member { member_id: $member_id, user_id: $user_id, user_name: $user_name, referral_code: $referral_code, hierarchy: $hierarchy, description: $description, created_at: $created_at, deleted: $deleted }) RETURN m")
                .params([
                    ("member_id", form_data.member_id.to_string().to_owned()),
                    ("user_id", form_data.user_id.to_string().to_owned()),
                    ("user_name", form_data.user_name.to_owned()),
                    ("referral_code", form_data.referral_code.to_owned()),
                    ("hierarchy", form_data.hierarchy.to_string().to_owned()),
                    ("description", form_data.description.to_owned()),
                    ("created_at", form_data.created_at.to_string().to_owned()),
                    ("deleted", form_data.deleted.to_string().to_owned())
                ])
        ).await?;

        Ok(form_data.clone())
    }

    pub async fn update_member(user_id: Uuid, description: &str) -> Result<(), neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let _ = graph.execute(
            query("MATCH (m:Member {user_id: $user_id}) SET m.description = $description RETURN m")
                .params([
                    ("user_id", user_id.to_string().to_owned()),
                    ("description", description.to_owned()),
                ])
        ).await?;

        Ok(())
    }

    pub async fn _delete(user_id: Uuid) -> Result<(), neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let _ = graph.execute(
            query("MATCH (m:Member {user_id: $user_id}) SET m.deleted = $deleted RETURN m")
                .params([
                    ("user_id", user_id.to_string().to_owned()),
                    ("deleted", true.to_string().to_owned())
                ])
        ).await?;

        Ok(())
    }

    pub async fn create_relationship(user_id: Uuid, referrer_id: Uuid) -> Result<Relation, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let mut opt = graph.execute(
            query("MATCH (m1:Member { user_id: $user_id }) MATCH (m2:Member { user_id: $referrer_id }) CREATE (m1)-[r:REFERRED_BY]->(m2) RETURN r")
                .params([
                    ("user_id", user_id.to_string().to_owned()),
                    ("referrer_id", referrer_id.to_string().to_owned()),
                ])
        ).await.unwrap();

        match opt.next().await.unwrap() {
            None => Err(neo4rs::Error::ConnectionError),
            Some(r) => {
                let relation: Relation = r.get("r").unwrap();
                Ok(relation)
            }
        }
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;

    // let r = MemberDbMutation::create_relationship(
    //     Uuid::from_str("e7c4536f-e27d-474c-97cb-9e18e8338d10").unwrap(),
    //     Uuid::from_str("70ef92d3-a856-412b-997d-6c27b827d8ff").unwrap(),
    // ).await?;
    //
    // println!("{:?}", r);

    let member = member::Model {
        ..Default::default()
    };

    MemberDbMutation::create_member(member).await.expect("failed");

    Ok(())
}