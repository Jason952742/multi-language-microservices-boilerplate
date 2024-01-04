use std::str::FromStr;
use shared::neo4rs::{self, Node, query};
use uuid::Uuid;
use shared::neo4j::Neo4jPool;
use shared::{convert_to_bool, convert_to_i32, opt_to_uuid, string_to_datetime};
use crate::domain::entities::enums::MemberType;
use crate::domain::entities::member;

pub struct MemberDbQuery;

impl MemberDbQuery {

    pub async fn get_member_by_id(id: Uuid) -> Result<Option<member::Model>, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let mut result = graph.execute(
            query("MATCH (member: Member {user_id: $id}) RETURN member").param("id", id.to_string()))
            .await?;

        if let Ok(Some(_row)) = result.next().await {
            let node: Node = _row.get("member").unwrap();
            Ok(Option::from(node_to_member(node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_member_by_my_referrer_code(code: &str) -> Result<Option<member::Model>, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let mut result = graph.execute(
            query("MATCH (member: Member {my_referrer_code: $code}) RETURN member").param("code", code))
            .await?;

        if let Ok(Some(_row)) = result.next().await {
            let node: Node = _row.get("member").unwrap();
            Ok(Option::from(node_to_member(node)))
        } else {
            Ok(None)
        }
    }

    pub async fn check_member(id: Uuid) -> Result<bool, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let count = graph
            .execute(query("MATCH (n:Member {user_id: $id}) RETURN COUNT(n) AS n").param("id", id.to_string()))
            .await.unwrap()
            .next()
            .await.unwrap().unwrap()
            .get::<i64>("n").unwrap();

        Ok(count > 0)
    }

    pub async fn get_referral_member(user_id: Uuid) -> Result<Option<member::Model>, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let mut result = graph.execute(
            query("MATCH (a:Member {user_id: $user_id})-[:REFERRED_BY]->(r:Member) RETURN r")
                .param("user_id", user_id.to_string())).await?;

        if let Ok(Some(_row)) = result.next().await {
            let node: Node = _row.get("r").unwrap();
            Ok(Option::from(node_to_member(node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_my_referees(user_id: Uuid) -> Result<Vec<member::Model>, neo4rs::Error> {
        let graph = Neo4jPool::graph().await;

        let mut result = graph.execute(
            query("MATCH (a:Member)-[:REFERRED_BY]->(b:Member {user_id: $user_id}) RETURN a")
                .param("user_id", user_id.to_string())).await?;

        let mut nodes = Vec::new();

        while let Ok(Some(_row)) = result.next().await {
            let node: Node = _row.get("a").unwrap();
            nodes.push(node);
        }

        println!("{:?}", nodes);

        let referees = nodes.into_iter().map(|x| node_to_member(x)).collect();

        Ok(referees)
    }
}

fn node_to_member(node: Node) -> member::Model {
    member::Model {
        user_id: opt_to_uuid(node.get::<String>("user_id")),
        user_name: node.get("user_name").unwrap(),
        member_type: MemberType::from_str(&node.get::<String>("member_type").unwrap()).unwrap(),
        member_id: opt_to_uuid(node.get::<String>("member_id")),
        login_creds: node.get::<String>("login_creds").unwrap(),
        level: convert_to_i32(node.get::<String>("level")),
        my_referrer_code: node.get::<String>("my_referrer_code").unwrap(),
        referee_code: node.get::<String>("referee_code").unwrap(),
        hierarchy: convert_to_i32(node.get::<String>("hierarchy")),
        active: convert_to_bool(node.get::<String>("active")),
        created_at: string_to_datetime(node.get::<String>("created_at")),
        updated_at: string_to_datetime(node.get::<String>("updated_at")),
        ..Default::default()
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let m = MemberQuery::get_member_by_id(Uuid::from_str("79cceea2-fa62-4689-b54b-d15ef5e96ce4").unwrap()).await?;
    //
    // println!("{:?}", m);
    //
    // let b = MemberQuery::check_member(Uuid::from_str("09cceea2-fa62-4689-b54b-d15ef5e96ce4").unwrap()).await?;
    //
    // println!("{:?}", b);
    //
    // let c = MemberQuery::get_member_by_my_referrer_code("6sZvOOyCQzSQft2vpk89UQ").await?;
    //
    // println!("{:?}", c);
    //
    // let r = MemberQuery::get_referral_member(Uuid::from_str("482b23eb-fdaf-498f-b4ac-ce39ecc6671d").unwrap()).await?;
    //
    // println!("{:?}", r);

    let rs = MemberDbQuery::get_my_referees(Uuid::from_str("79cceea2-fa62-4689-b54b-d15ef5e96ce4").unwrap()).await?;

    println!("{:?}", rs);

    Ok(())
}