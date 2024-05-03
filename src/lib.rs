pub mod error;

use crate::error::Error;
use client_api::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringInterval {
    Month,
    Year,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Pro,
    Team,
}

pub trait PaymentClient {
    fn stripe_payment_link(
        &self,
        workspace_id: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl PaymentClient for Client {
    async fn stripe_payment_link(
        &self,
        _workspace_id: &str,
        _recurring_interval: RecurringInterval,
        _workspace_subscription_plan: SubscriptionPlan,
    ) -> Result<String, Error> {
        todo!()
    }
}
