use super::*;
pub use crate::cli::GetVariables as Variables;
use crate::cli::IssueStatus;
use chrono::NaiveDateTime;
use serde::Deserialize;

pub struct GetIssue;
pub const OPERATION_NAME: &str = "GetIssue";
pub const QUERY : & str = "query GetIssue($id: Int!){\n  issue(issue: $id){\n    assignedTo,\n    createdAt,\n    createdBy,\n    description,\n    toOffline,\n    id,\n    status,\n    title,\n    comments{createdBy, comment, createdAt},\n    target{name, status},\n    related{name, status}\n  }\n}\n" ;
#[derive(Deserialize)]
pub struct ResponseData {
    pub issue: Option<GetIssueIssue>,
}
#[derive(Deserialize)]
pub struct GetIssueIssue {
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub description: String,
    #[serde(rename = "toOffline")]
    pub to_offline: Option<ToOffline>,
    pub id: i32,
    #[serde(rename = "status")]
    pub issue_status: IssueStatus,
    pub title: String,
    pub comments: Vec<GetIssueIssueComments>,
    pub target: Option<GetIssueIssueTarget>,
    pub related: Vec<GetIssueIssueTarget>,
}
#[derive(Deserialize)]
pub struct GetIssueIssueComments {
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub comment: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}
#[derive(Deserialize)]
pub struct GetIssueIssueTarget {
    pub name: String,
    pub status: TargetStatus,
}
impl graphql_client::GraphQLQuery for GetIssue {
    type Variables = Variables;
    type ResponseData = ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
