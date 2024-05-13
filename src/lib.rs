pub mod client;
pub mod entities;
use client_api::error::AppResponseError;
use entities::{RecurringInterval, SubscriptionPlan, WorkspaceSubscriptionStatus, WorkspaceUsage};

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

    fn get_workspace_usage(
        &self,
        workspace_id: &str,
    ) -> impl std::future::Future<Output = Result<WorkspaceUsage, AppResponseError>> + Send;
}
