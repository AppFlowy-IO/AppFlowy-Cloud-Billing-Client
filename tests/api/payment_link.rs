use appflowy_cloud_billing_client::PaymentClient;
use client_api_test_util::generate_unique_registered_user_client;

#[tokio::test]
async fn test_payment_link() {
    let (client, _) = generate_unique_registered_user_client().await;
    let workspace_id = client
        .get_workspaces()
        .await
        .unwrap()
        .0
        .first()
        .unwrap()
        .workspace_id
        .to_string();

    let url = client
        .stripe_payment_link(
            &workspace_id,
            appflowy_cloud_billing_client::RecurringInterval::Month,
            appflowy_cloud_billing_client::SubscriptionPlan::Pro,
        )
        .await
        .unwrap();

    let a = client.base_url;

    //assert!(url.starts_with("https://checkout.stripe.com/"));
}
