use deboa_tests::utils::CA_CERT;

use crate::{
    cert::{Certificate, ContentEncoding},
    tests::SKIP_CERT_VERIFICATION,
    Client,
};

pub(crate) fn client_with_cert() -> Client {
    Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build()
}
