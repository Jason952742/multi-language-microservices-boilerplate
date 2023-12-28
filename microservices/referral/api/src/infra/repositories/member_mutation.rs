use neo4rs::query;
use rust_decimal::prelude::ToPrimitive;
use shared::neo4j::Neo4j;
use crate::domain::entities::member;

pub struct MemberMutation;

impl MemberMutation {
    pub async fn create_member(form_data: member::Model) -> Result<member::Model, Box<dyn std::error::Error>> {
        let graph = Neo4j::graph().await;

        // graph.run(query("CREATE (m:Member { user_id: $user_id, user_name: $user_name, member_type: $member_type, member_id: $member_id, login_creds: $login_creds, level: $level, my_referrer_code: $my_referrer_code, referee_code: $referee_code, hierarchy: $hierarchy, active: $active, description: $description, created_at: $created_at, updated_at: $updated_at, enabled: $enabled, version: $version, deleted: $deleted })").params([
        //     ("user_id", &form_data.user_id.to_string()),
        //     ("user_name", &form_data.user_name),
        //     ("member_type", &form_data.member_type.to_string()),
        //     ("member_id", &form_data.member_id.to_string()),
        //     ("login_creds", &form_data.login_creds),
        //     ("level", &form_data.level.to_string()),
        //     ("my_referrer_code", &form_data.my_referrer_code),
        //     ("referee_code", &form_data.referee_code),
        //     ("hierarchy", &form_data.hierarchy.to_string()),
        //     ("active", &form_data.active.to_string()),
        //     ("description", &form_data.description.unwrap_or_default()),
        //     ("created_at", &form_data.created_at.to_string()),
        //     ("updated_at", &form_data.updated_at.to_string()),
        //     ("enabled", &form_data.enabled.to_string()),
        //     ("version", &form_data.version.to_string()),
        //     ("deleted", &form_data.deleted.to_string())
        // ])).await.unwrap();

        graph.run(
            query("CREATE (m:Member {user_id: $user_id })")
                .params([
                    ("user_id", form_data.user_id.to_string()),
                ])
        ).await.unwrap();

        Ok(form_data.clone())
    }


    pub async fn update_member() {}
}