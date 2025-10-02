use super::emulation_imports::*;
use super::*;
use http2::*;
use tls::*;

macro_rules! tls_config {
    (1, $cipher_list:expr) => {
        SafariTlsConfig::builder().cipher_list($cipher_list).build()
    };
    (2, $cipher_list:expr, $sigalgs_list:expr) => {
        SafariTlsConfig::builder()
            .cipher_list($cipher_list)
            .sigalgs_list($sigalgs_list)
            .build()
    };
}

macro_rules! http2_config {
    (1) => {
        Http2Config::builder()
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10551295)
            .max_concurrent_streams(100)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (2) => {
        Http2Config::builder()
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10551295)
            .max_concurrent_streams(100)
            .enable_push(false)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (3) => {
        Http2Config::builder()
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10485760)
            .max_concurrent_streams(100)
            .enable_push(false)
            .unknown_setting8(true)
            .unknown_setting9(true)
            .headers_priority(NEW_HEADER_PRIORITY)
            .headers_pseudo_order(NEW_HEADERS_PSEUDO_ORDER)
            .settings_order(NEW_SETTINGS_ORDER)
            .build()
    };
    (4) => {
        Http2Config::builder()
            .initial_stream_window_size(4194304)
            .initial_connection_window_size(10551295)
            .max_concurrent_streams(100)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (5) => {
        Http2Config::builder()
            .initial_stream_window_size(4194304)
            .initial_connection_window_size(10551295)
            .max_concurrent_streams(100)
            .enable_push(false)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (6) => {
        Http2Config::builder()
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10485760)
            .max_concurrent_streams(100)
            .enable_push(false)
            .unknown_setting9(true)
            .headers_priority(NEW_HEADER_PRIORITY)
            .headers_pseudo_order(NEW_HEADERS_PSEUDO_ORDER)
            .settings_order(NEW_SETTINGS_ORDER)
            .build()
    };
}

#[inline]
fn header_initializer_for_15(ua: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(ua));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    #[cfg(all(feature = "gzip", feature = "deflate", feature = "brotli"))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers
}

#[inline]
fn header_initializer_for_16_17(ua: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    #[cfg(all(feature = "gzip", feature = "deflate", feature = "brotli"))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static(ua));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers
}

#[inline]
fn header_initializer_for_18(ua: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert(USER_AGENT, HeaderValue::from_static(ua));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("priority", HeaderValue::from_static("u=0, i"));
    #[cfg(all(feature = "gzip", feature = "deflate", feature = "brotli"))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers
}

mod tls {
    use super::tls_imports::*;

    pub const CURVES: &[SslCurve] = &[
        SslCurve::X25519,
        SslCurve::SECP256R1,
        SslCurve::SECP384R1,
        SslCurve::SECP521R1,
    ];

    pub const CIPHER_LIST_1: &str = join!(
        ":",
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA256",
        "TLS_RSA_WITH_AES_128_CBC_SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA"
    );

    pub const CIPHER_LIST_2: &str = join!(
        ":",
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA"
    );

    pub const SIGALGS_LIST: &str = join!(
        ":",
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "ecdsa_sha1",
        "rsa_pss_rsae_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512",
        "rsa_pkcs1_sha1"
    );

    pub const NEW_SIGALGS_LIST: &str = join!(
        ":",
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512",
        "rsa_pkcs1_sha1"
    );

    pub const CERT_COMPRESSION_ALGORITHM: &[CertCompressionAlgorithm] =
        &[CertCompressionAlgorithm::Zlib];

    #[derive(TypedBuilder)]
    pub struct SafariTlsConfig {
        #[builder(default = CURVES)]
        curves: &'static [SslCurve],

        #[builder(default = SIGALGS_LIST)]
        sigalgs_list: &'static str,

        cipher_list: &'static str,
    }

    impl From<SafariTlsConfig> for TlsConfig {
        fn from(val: SafariTlsConfig) -> Self {
            TlsConfig::builder()
                .session_ticket(false)
                .grease_enabled(true)
                .enable_ocsp_stapling(true)
                .enable_signed_cert_timestamps(true)
                .curves(val.curves)
                .sigalgs_list(val.sigalgs_list)
                .cipher_list(val.cipher_list)
                .min_tls_version(TlsVersion::TLS_1_0)
                .cert_compression_algorithm(CERT_COMPRESSION_ALGORITHM)
                .build()
        }
    }

    impl From<SafariTlsConfig> for Option<TlsConfig> {
        #[inline(always)]
        fn from(val: SafariTlsConfig) -> Self {
            Some(val.into())
        }
    }
}

mod http2 {
    use super::http2_imports::*;

    pub const HEADER_PRIORITY: (u32, u8, bool) = (0, 255, true);
    pub const NEW_HEADER_PRIORITY: (u32, u8, bool) = (0, 255, false);

    pub const HEADERS_PSEUDO_ORDER: [PseudoOrder; 4] = [Method, Scheme, Path, Authority];
    pub const NEW_HEADERS_PSEUDO_ORDER: [PseudoOrder; 4] = [Method, Scheme, Authority, Path];

    pub const SETTINGS_ORDER: [SettingsOrder; 8] = [
        HeaderTableSize,
        EnablePush,
        InitialWindowSize,
        MaxConcurrentStreams,
        MaxFrameSize,
        MaxHeaderListSize,
        UnknownSetting8,
        UnknownSetting9,
    ];
    pub const NEW_SETTINGS_ORDER: [SettingsOrder; 8] = [
        HeaderTableSize,
        EnablePush,
        MaxConcurrentStreams,
        InitialWindowSize,
        MaxFrameSize,
        MaxHeaderListSize,
        UnknownSetting8,
        UnknownSetting9,
    ];
}

macro_rules! mod_generator {
    ($mod_name:ident, $tls_config:expr, $http2_config:expr, $header_initializer:ident, $ua:expr) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline(always)]
            pub fn emulation(option: EmulationOption) -> EmulationProvider {
                let default_headers = if !option.skip_headers {
                    Some($header_initializer($ua))
                } else {
                    None
                };

                build_emulation(option, default_headers)
            }

            #[inline(always)]
            pub fn build_emulation(
                option: EmulationOption,
                default_headers: Option<HeaderMap>,
            ) -> EmulationProvider {
                EmulationProvider::builder()
                    .tls_config($tls_config)
                    .http2_config(conditional_http2!(option.skip_http2, $http2_config))
                    .default_headers(default_headers)
                    .build()
            }
        }
    };

    ($mod_name:ident, $build_emulation:expr, $header_initializer:ident, $ua:expr) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline(always)]
            pub fn emulation(option: EmulationOption) -> EmulationProvider {
                let default_headers = if !option.skip_headers {
                    Some($header_initializer($ua))
                } else {
                    None
                };

                $build_emulation(option, default_headers)
            }
        }
    };
}

mod_generator!(
    safari15_3,
    tls_config!(1, CIPHER_LIST_1),
    http2_config!(4),
    header_initializer_for_15,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.3 Safari/605.1.15"
);

mod_generator!(
    safari15_5,
    safari15_3::build_emulation,
    header_initializer_for_15,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.5 Safari/605.1.15"
);

mod_generator!(
    safari15_6_1,
    tls_config!(1, CIPHER_LIST_2),
    http2_config!(4),
    header_initializer_for_15,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6.1 Safari/605.1.15"
);

mod_generator!(
    safari16,
    safari15_6_1::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15"
);

mod_generator!(
    safari16_5,
    safari15_6_1::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"
);

mod_generator!(
    safari17_4_1,
    safari15_6_1::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15"
);

mod_generator!(
    safari_ios_16_5,
    tls_config!(1, CIPHER_LIST_2),
    http2_config!(1),
    header_initializer_for_16_17,
    "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Mobile/15E148 Safari/604.1"
);

mod_generator!(
    safari17_0,
    tls_config!(1, CIPHER_LIST_2),
    http2_config!(5),
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15"
);

mod_generator!(
    safari17_2_1,
    safari17_0::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15"
);

mod_generator!(
    safari17_5,
    safari17_0::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15"
);

mod_generator!(
    safari_ios_17_2,
    tls_config!(1, CIPHER_LIST_2),
    http2_config!(2),
    header_initializer_for_16_17,
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1"
);

mod_generator!(
    safari_ios_17_4_1,
    safari_ios_17_2::build_emulation,
    header_initializer_for_16_17,
    "Mozilla/5.0 (iPad; CPU OS 17_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Mobile/15E148 Safari/604.1"
);

mod_generator!(
    safari18,
    tls_config!(1, CIPHER_LIST_2),
    http2_config!(3),
    header_initializer_for_18,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15"
);

mod_generator!(
    safari_ipad_18,
    safari18::build_emulation,
    header_initializer_for_18,
    "Mozilla/5.0 (iPad; CPU OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1"
);

mod_generator!(
    safari_ios_18_1_1,
    safari18::build_emulation,
    header_initializer_for_18,
    "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1.1 Mobile/15E148 Safari/604.1"
);

mod_generator!(
    safari18_2,
    tls_config!(2, CIPHER_LIST_2, NEW_SIGALGS_LIST),
    http2_config!(3),
    header_initializer_for_18,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Safari/605.1.15"
);

mod_generator!(
    safari18_3,
    safari18_2::build_emulation,
    header_initializer_for_18,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3 Safari/605.1.15"
);

mod_generator!(
    safari18_3_1,
    safari18_2::build_emulation,
    header_initializer_for_18,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3.1 Safari/605.1.15"
);

mod_generator!(
    safari18_5,
    tls_config!(2, CIPHER_LIST_2, NEW_SIGALGS_LIST),
    http2_config!(6),
    header_initializer_for_18,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15"
);
