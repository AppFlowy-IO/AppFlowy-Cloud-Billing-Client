use appflowy_cloud_billing_client::{
    entities::{RecurringInterval, SubscriptionPlan},
    WorkspaceSubscriptionClient,
};
use client_api_test_util::localhost_client;

#[tokio::test]
async fn test_payment_link() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password("zack@appflowy.io", "password")
        .await
        .unwrap();

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
    // assert!(url.starts_with("https://checkout.stripe.com/"));
    panic!("{:?}", url);
}

#[tokio::test]
async fn test_get_subscription() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password("zack@appflowy.io", "password")
        .await
        .unwrap();

    let subscriptions = client.list_subscription().await.unwrap();
    panic!("{:#?}", subscriptions);
}

#[tokio::test]
async fn test_cancel_subscription() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password(
            "user_b883b824-91a1-486d-94bc-09322dc0a4b1@appflowy.io",
            "password",
        )
        .await
        .unwrap();

    let workspace_id = client
        .get_workspaces()
        .await
        .unwrap()
        .0
        .first()
        .unwrap()
        .workspace_id
        .to_string();

    client.cancel_subscription(&workspace_id).await.unwrap();
}

#[tokio::test]
async fn test_get_limit() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password(
            "user_b883b824-91a1-486d-94bc-09322dc0a4b1@appflowy.io",
            "password",
        )
        .await
        .unwrap();

    let workspace_id = client
        .get_workspaces()
        .await
        .unwrap()
        .0
        .first()
        .unwrap()
        .workspace_id
        .to_string();

    client.cancel_subscription(&workspace_id).await.unwrap();
}
