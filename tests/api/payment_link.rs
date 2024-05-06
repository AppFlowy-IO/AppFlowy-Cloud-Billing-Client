use appflowy_cloud_billing_client::{
    entities::{RecurringInterval, SubscriptionPlan},
    WorkspaceSubscriptionClient,
};
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
        .create_subscription(
            &workspace_id,
            RecurringInterval::Month,
            SubscriptionPlan::Pro,
            "https://appflowy.io",
        )
        .await
        .unwrap();
    assert!(url.starts_with("https://checkout.stripe.com/"));
}
