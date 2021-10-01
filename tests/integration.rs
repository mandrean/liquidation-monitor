use liquidation_monitor::mantle::client::{MantleClient, MantleExt};

#[tokio::test]
async fn can_query_mantle_liquidation_price() {
    let liq_price =
        MantleClient::query_liquidation_price("terra1fhv4r0rm43cznxyxf0uv8jl4eapgn3tnq5dntv").await;

    assert!(liq_price.is_ok());
}

#[tokio::test]
async fn can_query_loan() {
    let loan = MantleClient::query_loan("terra1m6lg87pz0e54wzvhn09jqgec0xk8u2229rlxkv").await;

    assert!(loan.is_ok());
}
