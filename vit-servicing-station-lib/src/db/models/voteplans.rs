use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Queryable)]
pub struct Voteplan {
    pub id: i32,
    #[serde(alias = "chainVoteplanId")]
    pub chain_voteplan_id: String,
    #[serde(alias = "chainVoteStartTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_start_time: i64,
    #[serde(alias = "chainVoteEndTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_end_time: i64,
    #[serde(alias = "chainCommitteeEnd")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_committee_end: i64,
    #[serde(alias = "chainVoteplanPayload")]
    pub chain_voteplan_payload: String,
    #[serde(alias = "fundId")]
    pub fund_id: i32,
}

#[cfg(test)]
pub mod test {
    use crate::db::{models::voteplans::Voteplan, schema::voteplans, DBConnectionPool};

    use chrono::Utc;
    use diesel::{ExpressionMethods, RunQueryDsl};

    pub fn get_test_voteplan_with_fund_id(fund_id: i32) -> Voteplan {
        Voteplan {
            id: 1,
            chain_voteplan_id: "test_vote_plan".to_string(),
            chain_vote_start_time: Utc::now().timestamp(),
            chain_vote_end_time: Utc::now().timestamp(),
            chain_committee_end: Utc::now().timestamp(),
            chain_voteplan_payload: "foopayload".to_string(),
            fund_id,
        }
    }

    pub fn populate_db_with_voteplan(voteplan: &Voteplan, pool: &DBConnectionPool) {
        let connection = pool.get().unwrap();
        let values = (
            voteplans::chain_voteplan_id.eq(voteplan.chain_voteplan_id.clone()),
            voteplans::chain_vote_start_time.eq(voteplan.chain_vote_start_time),
            voteplans::chain_vote_end_time.eq(voteplan.chain_vote_end_time),
            voteplans::chain_committee_end_time.eq(voteplan.chain_committee_end),
            voteplans::chain_voteplan_payload.eq(voteplan.chain_voteplan_payload.clone()),
            voteplans::fund_id.eq(voteplan.fund_id),
        );
        diesel::insert_into(voteplans::table)
            .values(values)
            .execute(&connection)
            .unwrap();
    }
}