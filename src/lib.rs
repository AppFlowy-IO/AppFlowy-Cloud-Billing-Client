pub mod entities;
use crate::entities::WorkspaceUsageLimit;
use client_api::error::AppResponseError;
use entities::{RecurringInterval, SubscriptionPlan, WorkspaceSubscriptionStatus, WorkspaceUsage};
use reqwest::Method;
use serde_json::json;
use shared_entity::response::AppResponse;

pub fn billing_base_url(client: &client_api::Client) -> &str {
    // "http://localhost:4242"
    &client.base_url
}

pub async fn create_subscription(
    client: &client_api::Client,
    workspace_id: &str,
    recurring_interval: RecurringInterval,
    workspace_subscription_plan: SubscriptionPlan,
    success_url: &str,
) -> Result<String, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/subscription-link",
        billing_base_url(client),
    );

    let resp = client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .query(&[
            ("workspace_id", workspace_id),
            ("recurring_interval", recurring_interval.as_str()),
            (
                "workspace_subscription_plan",
                &workspace_subscription_plan.as_str(),
            ),
            ("success_url", success_url),
        ])
        .send()
        .await?;

    AppResponse::<String>::from_response(resp)
        .await?
        .into_data()
}

pub async fn cancel_subscription(
    client: &client_api::Client,
    workspace_id: &str,
) -> Result<(), AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/cancel-subscription",
        billing_base_url(client),
    );
    let resp = client
        .http_client_with_auth(Method::POST, &url)
        .await?
        .json(&json!({ "workspace_id": workspace_id }))
        .send()
        .await?;
    AppResponse::<()>::from_response(resp).await?.into_error()
}

pub async fn list_subscription(
    client: &client_api::Client,
) -> Result<Vec<WorkspaceSubscriptionStatus>, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/subscription-status",
        billing_base_url(client),
    );
    let resp = client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .send()
        .await?;

    AppResponse::<Vec<WorkspaceSubscriptionStatus>>::from_response(resp)
        .await?
        .into_data()
}

pub async fn get_workspace_usage(
    client: &client_api::Client,
    workspace_id: &str,
) -> Result<WorkspaceUsage, AppResponseError> {
    let num_members = client.get_workspace_members(workspace_id).await?.len();
    let limits = get_workspace_limits(client, workspace_id).await?;
    let doc_usage = client.get_workspace_usage(workspace_id).await?;

    let workspace_usage = WorkspaceUsage {
        member_count: num_members,
        member_count_limit: limits.member_count,
        total_blob_bytes: doc_usage.consumed_capacity as _,
        total_blob_bytes_limit: limits.total_blob_size,
    };
    Ok(workspace_usage)
}

pub async fn get_portal_session_link(
    client: &client_api::Client,
) -> Result<String, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/portal-session-link",
        billing_base_url(client),
    );
    let portal_url = client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .send()
        .await?
        .error_for_status()?
        .json::<AppResponse<String>>()
        .await?
        .into_data()?;
    Ok(portal_url)
}

async fn get_workspace_limits(
    client: &client_api::Client,
    workspace_id: &str,
) -> Result<WorkspaceUsageLimit, AppResponseError> {
    let url = format!("{}/api/workspace/{}/limit", &client.base_url, workspace_id);
    client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .send()
        .await?
        .error_for_status()?
        .json::<AppResponse<WorkspaceUsageLimit>>()
        .await?
        .into_data()
}
