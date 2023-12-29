
use neo4rs::{query, Relation};
use uuid::Uuid;
use shared::neo4j::Neo4j;
use crate::domain::entities::member;

pub struct MemberDbMutation;

impl MemberDbMutation {
    pub async fn create_member(form_data: member::Model) -> Result<member::Model, neo4rs::Error> {
        let graph = Neo4j::graph().await;

        let _ = graph.run(
            query("CREATE (m:Member { user_id: $user_id, user_name: $user_name, member_type: $member_type, member_id: $member_id, login_creds: $login_creds, level: $level, my_referrer_code: $my_referrer_code, referee_code: $referee_code, hierarchy: $hierarchy, active: $active, description: $description, created_at: $created_at, updated_at: $updated_at, enabled: $enabled, version: $version, deleted: $deleted }) RETURN m")
                .params([
                    ("user_id", form_data.user_id.to_string().to_owned()),
                    ("user_name", form_data.user_name.to_owned()),
                    ("member_type", form_data.member_type.to_string().to_owned()),
                    ("member_id", form_data.member_id.to_string().to_owned()),
                    ("login_creds", form_data.login_creds.to_owned()),
                    ("level", form_data.level.to_string().to_owned()),
                    ("my_referrer_code", form_data.my_referrer_code.to_owned()),
                    ("referee_code", form_data.referee_code.to_owned()),
                    ("hierarchy", form_data.hierarchy.to_string().to_owned()),
                    ("active", form_data.active.to_string().to_owned()),
                    ("description", form_data.description.to_owned()),
                    ("created_at", form_data.created_at.to_string().to_owned()),
                    ("updated_at", form_data.updated_at.to_string().to_owned()),
                    ("enabled", form_data.enabled.to_string().to_owned()),
                    ("version", form_data.version.to_string().to_owned()),
                    ("deleted", form_data.deleted.to_string().to_owned())
                ])
        ).await?;

        Ok(form_data.clone())
    }

    pub async fn update_member(form_data: member::Model) -> Result<member::Model, neo4rs::Error> {
        let graph = Neo4j::graph().await;

        let _ = graph.execute(
            query("MATCH (m:Member {user_id: user_id}) SET m.member_type = $member_type, m.level = $level,  m.active = $active, m.description = $description, m.updated_at = $updated_at, m.enabled = $enabled, m.version = $version, m.deleted = $deleted RETURN m")
                .params([
                    ("user_id", form_data.user_id.to_string().to_owned()),
                    ("member_type", form_data.member_type.to_string().to_owned()),
                    ("level", form_data.level.to_string().to_owned()),
                    ("active", form_data.active.to_string().to_owned()),
                    ("description", form_data.description.to_owned()),
                    ("updated_at", form_data.updated_at.to_string().to_owned()),
                    ("enabled", form_data.enabled.to_string().to_owned()),
                    ("version", form_data.version.to_string().to_owned()),
                    ("deleted", form_data.deleted.to_string().to_owned())
                ])
        ).await?;

        Ok(form_data.clone())
    }

    pub async fn create_relationship(referee_id: Uuid, referrer_id: Uuid) -> Result<Relation, neo4rs::Error> {
        let graph = Neo4j::graph().await;

        let mut opt = graph.execute(
            query("MATCH (m1:Member { user_id: $referee_id }) MATCH (m2:Member { user_id: $referrer_id }) CREATE (m1)-[r:REFERRED_BY]->(m2) RETURN r")
                .params([
                    ("referee_id", referee_id.to_string()),
                    ("referrer_id", referrer_id.to_string())
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

    let r = MemberDbMutation::create_relationship(
        Uuid::from_str("e7c4536f-e27d-474c-97cb-9e18e8338d10").unwrap(),
        Uuid::from_str("70ef92d3-a856-412b-997d-6c27b827d8ff").unwrap(),
    ).await?;

    println!("{:?}", r);

    Ok(())
}