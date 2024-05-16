pub mod entities;
use crate::entities::WorkspaceUsageLimit;
use entities::{RecurringInterval, SubscriptionPlan, WorkspaceSubscriptionStatus};
use reqwest::Method;
use serde_json::json;
use shared_entity::response::AppResponse;
use shared_entity::response::AppResponseError;

pub trait IsCloudClient {
    fn http_client_with_auth(
        &self,
        method: Method,
        url: &str,
    ) -> impl std::future::Future<Output = Result<reqwest::RequestBuilder, AppResponseError>> + Send;
    fn base_url(&self) -> &str;
}

pub trait WorkspaceSubscriptionClient: IsCloudClient {
    fn billing_base_url(&self) -> &str {
        self.base_url()
        // "http://localhost:4242"
    }

    fn create_subscription(
        &self,
        workspace_id: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
        success_url: &str,
    ) -> impl std::future::Future<Output = Result<String, AppResponseError>> + Send
    where
        Self: Sync,
    {
        create_subscription(
            self,
            workspace_id,
            recurring_interval,
            workspace_subscription_plan,
            success_url,
        )
    }

    fn list_subscription(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<WorkspaceSubscriptionStatus>, AppResponseError>>
           + Send
    where
        Self: Sync,
    {
        list_subscription(self)
    }

    fn cancel_subscription(
        &self,
        workspace_id: &str,
    ) -> impl std::future::Future<Output = Result<(), AppResponseError>> + Send
    where
        Self: Sync,
    {
        cancel_subscription(self, workspace_id)
    }

    fn get_workspace_limit(
        &self,
        workspace_id: &str,
    ) -> impl std::future::Future<Output = Result<WorkspaceUsageLimit, AppResponseError>> + Send
    where
        Self: Sync,
    {
        get_workspace_limit(self, workspace_id)
    }

    fn get_portal_session_link(
        &self,
    ) -> impl std::future::Future<Output = Result<String, AppResponseError>> + Send
    where
        Self: Sync,
    {
        get_portal_session_link(self)
    }
}

async fn create_subscription(
    billing_client: &(impl WorkspaceSubscriptionClient + ?Sized),
    workspace_id: &str,
    recurring_interval: RecurringInterval,
    workspace_subscription_plan: SubscriptionPlan,
    success_url: &str,
) -> Result<String, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/subscription-link",
        &billing_client.billing_base_url(),
    );

    let resp = billing_client
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

async fn list_subscription(
    billing_client: &(impl WorkspaceSubscriptionClient + ?Sized),
) -> Result<Vec<WorkspaceSubscriptionStatus>, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/subscription-status",
        billing_client.billing_base_url(),
    );
    let resp = billing_client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .send()
        .await?;

    AppResponse::<Vec<WorkspaceSubscriptionStatus>>::from_response(resp)
        .await?
        .into_data()
}

async fn cancel_subscription(
    billing_client: &(impl WorkspaceSubscriptionClient + ?Sized),
    workspace_id: &str,
) -> Result<(), AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/cancel-subscription",
        billing_client.billing_base_url(),
    );
    let resp = billing_client
        .http_client_with_auth(Method::POST, &url)
        .await?
        .json(&json!({ "workspace_id": workspace_id }))
        .send()
        .await?;
    AppResponse::<()>::from_response(resp).await?.into_error()
}

async fn get_workspace_limit(
    billing_client: &(impl WorkspaceSubscriptionClient + ?Sized),
    workspace_id: &str,
) -> Result<WorkspaceUsageLimit, AppResponseError> {
    let url = format!(
        "{}/api/workspace/{}/limit",
        billing_client.billing_base_url(),
        workspace_id
    );
    billing_client
        .http_client_with_auth(Method::GET, &url)
        .await?
        .send()
        .await?
        .error_for_status()?
        .json::<AppResponse<WorkspaceUsageLimit>>()
        .await?
        .into_data()
}

async fn get_portal_session_link(
    billing_client: &(impl WorkspaceSubscriptionClient + ?Sized),
) -> Result<String, AppResponseError> {
    let url = format!(
        "{}/billing/api/v1/portal-session-link",
        billing_client.billing_base_url(),
    );
    let portal_url = billing_client
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
