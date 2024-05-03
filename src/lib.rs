use client_api::Client;

pub enum RecurringInterval {
    Month,
    Year,
}

pub enum SubscriptionPlan {
    Pro,
    Team,
}

pub trait PaymentClient {
    async fn stripe_payment_link(
        &self,
        workspace_id: &str,
        success_url: &str,
        recurring_interval: RecurringInterval,
        workspace_subscription_plan: SubscriptionPlan,
    ) -> Result<String, String>;
}

impl PaymentClient for Client {
    async fn stripe_payment_link(
        &self,
        _workspace_id: &str,
        _success_url: &str,
        _recurring_interval: RecurringInterval,
        _workspace_subscription_plan: SubscriptionPlan,
    ) -> Result<String, String> {
        todo!()
    }
}
