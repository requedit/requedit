// https://github.com/sinKettu/cruster/blob/master/src/utils.rs

use std::fs;
use std::io::Error;

use hudsucker::certificate_authority::RcgenAuthority;
use hudsucker::rustls::{Certificate, PrivateKey};
use rcgen::{
    BasicConstraints, Certificate as RCgenCertificate, CertificateParams, IsCa,
    PKCS_ECDSA_P256_SHA256,
};

use time::macros::datetime;
use time::OffsetDateTime;

pub(crate) fn generate_key_and_cer(key_path: &str, cer_path: &str) {
    if std::path::Path::new(key_path).exists() && std::path::Path::new(cer_path).exists() {
        return;
    }

    let mut cert_params = CertificateParams::default();

    cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    cert_params.not_before = OffsetDateTime::from(datetime!(1970-01-01 0:00 UTC));
    cert_params.not_after = OffsetDateTime::from(datetime!(5000-01-01 0:00 UTC));
    cert_params.key_pair = None;
    cert_params.alg = &PKCS_ECDSA_P256_SHA256;

    let new_cert = RCgenCertificate::from_params(cert_params).unwrap();
    fs::write(cer_path, new_cert.serialize_pem().unwrap()).unwrap();
    fs::write(key_path, new_cert.serialize_private_key_pem()).unwrap();
}

pub(crate) fn get_ca(key_path: &str, cer_path: &str) -> Result<RcgenAuthority, Error> {
    use std::io::Read;

    let mut key_buffer: Vec<u8> = Vec::new();
    let f = fs::File::open(key_path);
    match f {
        Ok(mut file) => {
            let res = file.read_to_end(&mut key_buffer);
            if let Err(e) = res {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    }

    let mut cer_buffer: Vec<u8> = Vec::new();
    let f = fs::File::open(cer_path);
    match f {
        Ok(mut file) => {
            let res = file.read_to_end(&mut cer_buffer);
            if let Err(e) = res {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    }

    return {
        let mut key_buffer_ref = key_buffer.as_slice();
        let mut cert_buffer_ref = cer_buffer.as_slice();

        let mut private_key_raw = rustls_pemfile::pkcs8_private_keys(&mut key_buffer_ref).unwrap();
        let mut ca_cert_raw = rustls_pemfile::certs(&mut cert_buffer_ref).unwrap();

        let private_key = PrivateKey(private_key_raw.remove(0));
        let ca_cert = Certificate(ca_cert_raw.remove(0));

        Ok(RcgenAuthority::new(private_key, ca_cert, 1000).unwrap())
    };
}