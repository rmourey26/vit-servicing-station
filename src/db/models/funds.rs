use crate::db::{models::voteplans::Voteplan, schema::funds, DB};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Fund {
    pub id: i32,
    pub fund_name: String,
    pub fund_goal: String,
    pub voting_power_info: String,
    pub rewards_info: String,
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub fund_start_time: i64,
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub fund_end_time: i64,
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub next_fund_start_time: i64,
    pub chain_vote_plans: Vec<Voteplan>,
}

impl Queryable<funds::SqlType, DB> for Fund {
    type Row = (
        // 0 -> id
        i32,
        // 1 -> fund_name
        String,
        // 2 -> fund_goal
        String,
        // 3 -> voting_power_info
        String,
        // 4 -> rewards_info
        String,
        // 5 -> fund_start_time
        i64,
        // 6 -> fund_end_time
        i64,
        // 7 -> next_fund_start_time
        i64,
    );

    fn build(row: Self::Row) -> Self {
        Fund {
            id: row.0,
            fund_name: row.1,
            fund_goal: row.2,
            voting_power_info: row.3,
            rewards_info: row.4,
            fund_start_time: row.5,
            fund_end_time: row.6,
            next_fund_start_time: row.7,
            chain_vote_plans: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::{models::funds::Fund, schema::funds, DBConnectionPool};

    use chrono::Utc;
    use diesel::{ExpressionMethods, RunQueryDsl};

    pub fn get_test_fund() -> Fund {
        Fund {
            id: 1,
            fund_name: "hey oh let's go".to_string(),
            fund_goal: "test this endpoint".to_string(),
            voting_power_info: ">9000".to_string(),
            rewards_info: "not much".to_string(),
            fund_start_time: Utc::now().timestamp(),
            fund_end_time: Utc::now().timestamp(),
            next_fund_start_time: Utc::now().timestamp(),
            chain_vote_plans: vec![],
        }
    }

    pub fn populate_db_with_fund(fund: &Fund, pool: &DBConnectionPool) {
        let connection = pool.get().unwrap();
        let values = (
            funds::fund_name.eq(fund.fund_name.clone()),
            funds::fund_goal.eq(fund.fund_goal.clone()),
            funds::voting_power_info.eq(fund.voting_power_info.clone()),
            funds::rewards_info.eq(fund.rewards_info.clone()),
            funds::fund_start_time.eq(fund.fund_start_time),
            funds::fund_end_time.eq(fund.fund_end_time),
            funds::next_fund_start_time.eq(fund.next_fund_start_time),
        );
        diesel::insert_into(funds::table)
            .values(values)
            .execute(&connection)
            .unwrap();
    }
}
