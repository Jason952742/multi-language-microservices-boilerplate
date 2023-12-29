use neo4rs::query;
use shared::neo4j::Neo4j;
use crate::domain::entities::member;

pub struct MemberMutation;

impl MemberMutation {
    pub async fn create_member(form_data: member::Model) -> Result<member::Model, neo4rs::Error> {
        let graph = Neo4j::graph().await;

        graph.run(
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

        graph.run(
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
}