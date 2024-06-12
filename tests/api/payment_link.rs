use appflowy_cloud_billing_client::{
    entities::{RecurringInterval, SubscriptionPlan},
    BillingClient,
};
use client_api_test::{generate_unique_registered_user_client, localhost_client};

#[tokio::test]
async fn test_payment_link() {
    let (client, user) = generate_unique_registered_user_client().await;
    println!("User: {:?}", user);

    let workspace_id = client
        .get_workspaces()
        .await
        .unwrap()
        .0
        .first()
        .unwrap()
        .workspace_id
        .to_string();

    let url = BillingClient::from(&client)
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
        .sign_in_password(
            "user_ce5c12f5-3afa-456a-9fbc-492acee7c2e0@appflowy.io",
            "Hello123!",
        )
        .await
        .unwrap();

    let subscriptions = BillingClient::from(&client)
        .list_subscription()
        .await
        .unwrap();
    panic!("{:#?}", subscriptions);
}

#[tokio::test]
async fn test_cancel_subscription() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password(
            "user_9c064aff-ca1a-4063-8263-3fcaacb3adb8@appflowy.io",
            "Hello123!",
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

    BillingClient::from(&client)
        .cancel_subscription(&workspace_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_usage() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password(
            "user_9c064aff-ca1a-4063-8263-3fcaacb3adb8@appflowy.io",
            "Hello123!",
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

    let u = BillingClient::from(&client)
        .get_workspace_usage(&workspace_id)
        .await
        .unwrap();
    panic!("{:?}", u);
}

#[tokio::test]
async fn test_get_portal_link() {
    let client = localhost_client();
    client
        // registered and subscribed user (TODO: simulate subscribed user for automated testing)
        .sign_in_password(
            "user_ce5c12f5-3afa-456a-9fbc-492acee7c2e0@appflowy.io",
            "Hello123!",
        )
        .await
        .unwrap();

    let url = BillingClient::from(&client)
        .get_portal_session_link()
        .await
        .unwrap();
    panic!("{:?}", url);
}
