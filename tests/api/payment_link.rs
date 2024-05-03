use appflowy_cloud_billing_client::PaymentClient;

#[tokio::test]
async fn test_payment_link() {
    let client: client_api::Client = todo!();
    let _ = client.stripe_payment_link().await;
}
