use crate::{
    common::{init_test_bucket, rewrite_url, ClientHelper},
    scenario::batch_objects_nominal::batch_objects_nominal_proxy,
};

mod common;
mod scenario;

/**
 * Integration test for the nominal case
 */
#[tokio::test]
async fn test_batch_objects_nominal() {
    let (app, config) = ClientHelper::new(vec!["proxy", "sbs"]);
    init_test_bucket(&config).await;
    let custom_signer_host = config.custom_signer_host.unwrap();
    batch_objects_nominal_proxy(
        app,
        Box::new(move |url, repo| rewrite_url(url, repo, &custom_signer_host)),
    )
    .await;
}
