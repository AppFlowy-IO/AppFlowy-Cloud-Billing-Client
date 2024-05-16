use appflowy_cloud_billing_client::{IsCloudClient, WorkspaceSubscriptionClient};
use client_api::error::AppResponseError;
use client_api::Client;
use reqwest::Method;

mod api;

struct BillingClient {
    client: Client,
}

impl From<Client> for BillingClient {
    fn from(client: Client) -> Self {
        Self { client }
    }
}

impl IsCloudClient for BillingClient {
    fn base_url(&self) -> &str {
        &self.client.base_url
    }

    fn http_client_with_auth(
        &self,
        method: Method,
        url: &str,
    ) -> impl std::future::Future<Output = Result<reqwest::RequestBuilder, AppResponseError>> + Send
    {
        self.client.http_client_with_auth(method, url)
    }
}

impl WorkspaceSubscriptionClient for BillingClient {}
