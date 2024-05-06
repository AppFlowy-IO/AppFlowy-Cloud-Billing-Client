pub mod entities;
use client_api::Client;
use entities::{RecurringInterval, SubscriptionPlan};
use reqwest::Method;
use shared_entity::response::{AppResponse, AppResponseError};

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
    ) -> impl std::future::Future<Output = Result<(), AppResponseError>> + Send;

    fn cancel_subscription(
        &self,
        workspace_id: &str,
    ) -> impl std::future::Future<Output = Result<(), AppResponseError>> + Send;
}

impl WorkspaceSubscriptionClient for Client {
    fn billing_base_url(&self) -> &str {
        // local test: "http://localhost:4242"
        // &self.base_url
        "http://localhost:4242"
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
        todo!()
    }

    async fn list_subscription(&self) -> Result<(), AppResponseError> {
        todo!()
    }
}
