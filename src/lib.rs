use client_api::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use shared_entity::response::{AppResponse, AppResponseError};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringInterval {
    Month,
    Year,
}

impl RecurringInterval {
    fn as_str(&self) -> &str {
        match self {
            RecurringInterval::Month => "month",
            RecurringInterval::Year => "year",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Pro,
    Team,
}

impl SubscriptionPlan {
    fn as_str(&self) -> &str {
        match self {
            SubscriptionPlan::Pro => "pro",
            SubscriptionPlan::Team => "team",
        }
    }
}

pub trait PaymentClient {
    fn billing_base_url(&self) -> &str;
    fn stripe_payment_link(
        &self,
        workspace_id: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
        success_url: &str,
    ) -> impl std::future::Future<Output = Result<String, AppResponseError>> + Send;
}

impl PaymentClient for Client {
    async fn stripe_payment_link(
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

    fn billing_base_url(&self) -> &str {
        // local test: "http://localhost:4242"
        // &self.base_url
        "http://localhost:4242"
    }
}
