pub mod entities;
use client_api::{error::AppResponseError, Client};
use entities::{RecurringInterval, SubscriptionPlan, WorkspaceSubscriptionStatus};
use reqwest::Method;
use serde_json::json;
use shared_entity::response::AppResponse;

pub trait WorkspaceSubscriptionClient {
    fn billing_base_url(&self) -> &str;

    fn create_subscription(
        &self,
        workspace_id: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
        success_url: &str,
    ) -> impl std::future::Future<Output = Result<String, AppResponseError>> + Send;

    fn list_subscription(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<WorkspaceSubscriptionStatus>, AppResponseError>>
           + Send;

    fn cancel_subscription(
        &self,
        workspace_id: &str,
    ) -> impl std::future::Future<Output = Result<(), AppResponseError>> + Send;
}
impl WorkspaceSubscriptionClient for Client {
    fn billing_base_url(&self) -> &str {
        // "http://localhost:4242"
        &self.base_url
    }

    async fn create_subscription(
        &self,
        workspace_id: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
        success_url: &str,
    ) -> Result<String, AppResponseError> {
        let url = format!(
            "{}/billing/api/v1/subscription-link",
            &self.billing_base_url(),
        );

        let resp = self
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

    async fn cancel_subscription(&self, workspace_id: &str) -> Result<(), AppResponseError> {
        let url = format!(
            "{}/billing/api/v1/cancel-subscription",
            &self.billing_base_url(),
        );
        let resp = self
            .http_client_with_auth(Method::POST, &url)
            .await?
            .json(&json!({ "workspace_id": workspace_id }))
            .send()
            .await?;
        AppResponse::<()>::from_response(resp).await?.into_error()
    }

    async fn list_subscription(
        &self,
    ) -> Result<Vec<WorkspaceSubscriptionStatus>, AppResponseError> {
        let url = format!(
            "{}/billing/api/v1/subscription-status",
            &self.billing_base_url(),
        );
        let resp = self
            .http_client_with_auth(Method::GET, &url)
            .await?
            .send()
            .await?;

        AppResponse::<Vec<WorkspaceSubscriptionStatus>>::from_response(resp)
            .await?
            .into_data()
    }
}
