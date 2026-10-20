use crate::{TestResult, dns::DnsResponse};
use caramelo::{expect, matchers::eq};
use http::Version;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn test_valid_dns_response_builder() -> TestResult<()> {
    let response = DnsResponse::builder()
        .addresses(vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))])
        .protos(vec![Version::HTTP_11])
        .build();

    expect(response.addresses()).to_be(eq(&vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))]));
    expect(response.protos()).to_be(eq(&vec![Version::HTTP_11]));

    Ok(())
}
