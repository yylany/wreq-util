use super::emulation_imports::*;
use super::*;
use http2::*;
use tls::*;

macro_rules! tls_config {
    (1) => {
        ChromeTlsConfig::builder().build()
    };
    (2) => {
        ChromeTlsConfig::builder().enable_ech_grease(true).build()
    };
    (3) => {
        ChromeTlsConfig::builder().permute_extensions(true).build()
    };
    (4) => {
        ChromeTlsConfig::builder()
            .permute_extensions(true)
            .enable_ech_grease(true)
            .build()
    };
    (5) => {
        ChromeTlsConfig::builder()
            .permute_extensions(true)
            .enable_ech_grease(true)
            .pre_shared_key(true)
            .build()
    };
    (6, $curves:expr) => {
        ChromeTlsConfig::builder()
            .curves($curves)
            .permute_extensions(true)
            .pre_shared_key(true)
            .enable_ech_grease(true)
            .build()
    };
    (7, $curves:expr) => {
        ChromeTlsConfig::builder()
            .curves($curves)
            .permute_extensions(true)
            .pre_shared_key(true)
            .enable_ech_grease(true)
            .alps_use_new_codepoint(true)
            .build()
    };
}

macro_rules! http2_config {
    (1) => {
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_concurrent_streams(1000)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (2) => {
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_concurrent_streams(1000)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .enable_push(false)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
    (3) => {
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .enable_push(false)
            .headers_priority(HEADER_PRIORITY)
            .headers_pseudo_order(HEADERS_PSEUDO_ORDER)
            .settings_order(SETTINGS_ORDER)
            .build()
    };
}

#[inline]
fn header_initializer(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    header_chrome_ua!(headers, ua);
    header_chrome_sec_fetch!(headers);
    header_chrome_accpet!(headers);
    headers
}

#[inline]
fn header_initializer_with_zstd(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    header_chrome_ua!(headers, ua);
    header_chrome_sec_fetch!(headers);
    header_chrome_accpet!(zstd, headers);
    headers
}

#[inline]
fn header_initializer_with_zstd_priority(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    header_chrome_ua!(headers, ua);
    header_chrome_sec_fetch!(headers);
    header_chrome_accpet!(zstd, headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    headers
}

mod tls {
    use super::tls_imports::*;

    pub const CURVES_1: &[SslCurve] = &[SslCurve::X25519, SslCurve::SECP256R1, SslCurve::SECP384R1];

    pub const CURVES_2: &[SslCurve] = &[
        SslCurve::X25519_KYBER768_DRAFT00,
        SslCurve::X25519,
        SslCurve::SECP256R1,
        SslCurve::SECP384R1,
    ];

    pub const CURVES_3: &[SslCurve] = &[
        SslCurve::X25519_MLKEM768,
        SslCurve::X25519,
        SslCurve::SECP256R1,
        SslCurve::SECP384R1,
    ];

    pub const CIPHER_LIST: &str = join!(
        ":",
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA"
    );

    pub const SIGALGS_LIST: &str = join!(
        ":",
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512"
    );

    pub const CERT_COMPRESSION_ALGORITHM: &[CertCompressionAlgorithm] =
        &[CertCompressionAlgorithm::Brotli];

    #[derive(TypedBuilder)]
    pub struct ChromeTlsConfig {
        #[builder(default = CURVES_1)]
        curves: &'static [SslCurve],

        #[builder(default = SIGALGS_LIST)]
        sigalgs_list: &'static str,

        #[builder(default = CIPHER_LIST)]
        cipher_list: &'static str,

        #[builder(default = AlpsProtos::HTTP2, setter(into))]
        alps_protos: AlpsProtos,

        #[builder(default = false)]
        alps_use_new_codepoint: bool,

        #[builder(default = false, setter(into))]
        enable_ech_grease: bool,

        #[builder(default = false, setter(into))]
        permute_extensions: bool,

        #[builder(default = false, setter(into))]
        pre_shared_key: bool,
    }

    impl From<ChromeTlsConfig> for TlsConfig {
        fn from(val: ChromeTlsConfig) -> Self {
            TlsConfig::builder()
                .grease_enabled(true)
                .enable_ocsp_stapling(true)
                .enable_signed_cert_timestamps(true)
                .curves(val.curves)
                .sigalgs_list(val.sigalgs_list)
                .cipher_list(val.cipher_list)
                .min_tls_version(TlsVersion::TLS_1_2)
                .max_tls_version(TlsVersion::TLS_1_3)
                .permute_extensions(val.permute_extensions)
                .pre_shared_key(val.pre_shared_key)
                .enable_ech_grease(val.enable_ech_grease)
                .alps_protos(val.alps_protos)
                .alps_use_new_codepoint(val.alps_use_new_codepoint)
                .cert_compression_algorithm(CERT_COMPRESSION_ALGORITHM)
                .build()
        }
    }

    impl From<ChromeTlsConfig> for Option<TlsConfig> {
        #[inline(always)]
        fn from(val: ChromeTlsConfig) -> Self {
            Some(val.into())
        }
    }
}

mod http2 {
    use super::http2_imports::*;

    pub const HEADER_PRIORITY: (u32, u8, bool) = (0, 255, true);

    pub const HEADERS_PSEUDO_ORDER: [PseudoOrder; 4] = [Method, Authority, Scheme, Path];

    pub const SETTINGS_ORDER: [SettingsOrder; 8] = [
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
    (
        $mod_name:ident,
        $tls_config:expr,
        $http2_config:expr,
        $header_initializer:ident,
        [($default_os:ident, $default_sec_ch_ua:tt, $default_ua:tt) $(, ($other_os:ident, $other_sec_ch_ua:tt, $other_ua:tt))*]
    ) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline(always)]
            pub fn emulation(option: EmulationOption) -> EmulationProvider {
                let default_headers = if !option.skip_headers {
                    #[allow(unreachable_patterns)]
                    let default_headers = match option.emulation_os {
                        $(
                            EmulationOS::$other_os => $header_initializer(
                                $other_sec_ch_ua,
                                $other_ua,
                                option.emulation_os,
                            ),
                        )*
                        _ => $header_initializer(
                            $default_sec_ch_ua,
                            $default_ua,
                            EmulationOS::$default_os,
                        ),
                    };
                    Some(default_headers)
                } else {
                    None
                };

                build_emulation(option, default_headers)
            }

            #[inline(always)]
            pub fn build_emulation(
                option: EmulationOption,
                default_headers: Option<HeaderMap>
            ) -> EmulationProvider {
                EmulationProvider::builder()
                     .tls_config($tls_config)
                    .http2_config(conditional_http2!(option.skip_http2, $http2_config))
                    .default_headers(default_headers)
                    .build()
            }
        }
    };
    (
        $mod_name:ident,
        $build_emulation:expr,
        $header_initializer:ident,
        [($default_os:ident, $default_sec_ch_ua:tt, $default_ua:tt) $(, ($other_os:ident, $other_sec_ch_ua:tt, $other_ua:tt))*]
    ) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline(always)]
            pub fn emulation(option: EmulationOption) -> EmulationProvider {
                let default_headers = if !option.skip_headers {
                    #[allow(unreachable_patterns)]
                    let default_headers = match option.emulation_os {
                        $(
                            EmulationOS::$other_os => $header_initializer(
                                $other_sec_ch_ua,
                                $other_ua,
                                option.emulation_os,
                            ),
                        )*
                        _ => $header_initializer(
                            $default_sec_ch_ua,
                            $default_ua,
                            EmulationOS::$default_os,
                        ),
                    };
                    Some(default_headers)
                } else {
                    None
                };

                $build_emulation(option, default_headers)
            }
        }
    };
}

mod_generator!(
    v100,
    tls_config!(1),
    http2_config!(1),
    header_initializer,
    [
        (
            MacOS,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Android,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Windows,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 15_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/100.0.4896.85 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v101,
    v100::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Android,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Windows,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/101.0.4951.58 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    edge101,
    v100::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.47"
        ),
        (
            Android,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
            "Mozilla/5.0 (Linux; Android 10; ONEPLUS A6003) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36 Edg/101.0.1210.31"
        ),
        (
            Windows,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.53"
        ),
        // This shouldn't exist, edge was never meant to be on linux,
        // but I found some UAs in myip.ms (same for 122, 127 and 131)
        (
            Linux,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.53"
        ),
        (
            IOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/101.0.4951.64 Mobile Safari/537.36 Edg/101.0.1210.53"
        )
    ]
);

mod_generator!(
    v104,
    v100::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/104.0.5112.99 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v105,
    tls_config!(2),
    http2_config!(1),
    header_initializer,
    [
        (
            MacOS,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/105.0.5195.100 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v106,
    tls_config!(3),
    http2_config!(2),
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/106.0.5249.92 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v107,
    v106::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="107", "Google Chrome";v="107", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="107", "Google Chrome";v="107", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="107", "Google Chrome";v="107", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="107", "Google Chrome";v="107", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="107", "Google Chrome";v="107", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/107.0.5304.66 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v108,
    v106::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Not?A_Brand";v="108", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not?A_Brand";v="108", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not?A_Brand";v="108", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not?A_Brand";v="108", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/108.0.5359.112 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v109,
    v106::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="109", "Google Chrome";v="109", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="109", "Google Chrome";v="109", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="109", "Google Chrome";v="109", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="109", "Google Chrome";v="109", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="109", "Google Chrome";v="109", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/109.0.5414.112 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v110,
    v100::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="110", "Google Chrome";v="110", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not?A_Brand";v="110", "Chromium";v="110", "Google Chrome";v="110""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not?A_Brand";v="110", "Chromium";v="110", "Google Chrome";v="110""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not?A_Brand";v="110", "Chromium";v="110", "Google Chrome";v="110""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not?A_Brand";v="110", "Chromium";v="110", "Google Chrome";v="110""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.104 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v114,
    v106::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="114", "Google Chrome";v="114", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="114", "Google Chrome";v="114", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="114", "Google Chrome";v="114", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="114", "Google Chrome";v="114", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        )
    ]
);

mod_generator!(
    v116,
    tls_config!(4),
    http2_config!(2),
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="116", "Google Chrome";v="116", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="116", "Google Chrome";v="116", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="116", "Google Chrome";v="116", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="116", "Google Chrome";v="116", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="116", "Google Chrome";v="116", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/116.0.5845.103 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v117,
    tls_config!(5),
    http2_config!(3),
    header_initializer,
    [
        (
            MacOS,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        )
    ]
);

mod_generator!(
    v120,
    v117::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="120", "Google Chrome";v="120", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="120", "Google Chrome";v="120", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="120", "Google Chrome";v="120", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="120", "Google Chrome";v="120", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="120", "Google Chrome";v="120", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/120.0.6099.119 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    edge122,
    v117::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"
        ),
        (
            Android,
            r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
            "Mozilla/5.0 (Linux; Android 10; Pixel 3 XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6268.219 Safari/537.36 Edg/122.0.2238.82"
        ),
        (
            Windows,
            r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"
        ),
        // This shouldn't exist, edge was never meant to be on linux
        (
            Linux,
            r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"
        ),
        (
            IOS,
            r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"
        )
    ]
);

mod_generator!(
    v123,
    v117::build_emulation,
    header_initializer_with_zstd,
    [
        (
            MacOS,
            r#""Google Chrome";v="123", "Not;A=Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="123", "Not;A=Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="123", "Not;A=Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="123", "Not;A=Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        )
    ]
);

mod_generator!(
    v118,
    tls_config!(4),
    http2_config!(3),
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/118.0.5993.92 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v119,
    v118::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="119", "Google Chrome";v="119", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="119", "Google Chrome";v="119", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="119", "Google Chrome";v="119", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="119", "Google Chrome";v="119", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="119", "Google Chrome";v="119", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/119.0.6045.109 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v124,
    tls_config!(6, CURVES_2),
    http2_config!(3),
    header_initializer_with_zstd,
    [
        (
            MacOS,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/124.0.6312.52 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v126,
    v124::build_emulation,
    header_initializer_with_zstd,
    [
        (
            MacOS,
            r#""Chromium";v="126", "Google Chrome";v="126", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="126", "Google Chrome";v="126", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="126", "Google Chrome";v="126", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="126", "Google Chrome";v="126", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="126", "Google Chrome";v="126", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/126.0.6478.153 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v127,
    v124::build_emulation,
    header_initializer_with_zstd,
    [
        (
            MacOS,
            r#""Not/A)Brand";v="8", "Chromium";v="127", "Google Chrome";v="127""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not/A)Brand";v="8", "Chromium";v="127", "Google Chrome";v="127""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not/A)Brand";v="8", "Chromium";v="127", "Google Chrome";v="127""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not/A)Brand";v="8", "Chromium";v="127", "Google Chrome";v="127""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not/A)Brand";v="8", "Chromium";v="127", "Google Chrome";v="127""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/127.0.6533.77 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    edge127,
    v124::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"
        ),
        (
            Android,
            r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.6332.205 Safari/537.36 Edg/127.0.2322.67"
        ),
        (
            Windows,
            r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"
        ),
        // This shouldn't exist, edge was never meant to be on linux
        (
            Linux,
            r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"
        ),
        (
            IOS,
            r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"
        )
    ]
);

mod_generator!(
    v128,
    v124::build_emulation,
    header_initializer,
    [
        (
            MacOS,
            r#""Chromium";v="128", "Google Chrome";v="128", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="128", "Google Chrome";v="128", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="128", "Google Chrome";v="128", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="128", "Google Chrome";v="128", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="128", "Google Chrome";v="128", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/128.0.6613.98 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v129,
    v124::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="129", "Chromium";v="129", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="129", "Chromium";v="129", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="129", "Chromium";v="129", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="129", "Chromium";v="129", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="129", "Chromium";v="129", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/129.0.6668.46 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v130,
    v124::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (iPad; CPU OS 18_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/130.0.6723.90 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v131,
    tls_config!(6, CURVES_3),
    http2_config!(3),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/131.0.6778.134 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    edge131,
    v131::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
        ),
        (
            Android,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; HD1913) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.200 Mobile Safari/537.36 EdgA/131.0.2903.87"
        ),
        (
            Windows,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
        ),
        // This shouldn't exist, edge was never meant to be on linux
        (
            Linux,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
        ),
        (
            IOS,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
        )
    ]
);

mod_generator!(
    v132,
    tls_config!(7, CURVES_3),
    http2_config!(3),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/132.0.6834.78 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v133,
    v132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/133.0.6943.33 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v134,
    v132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/134.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    edge134,
    v131::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"
        ),
        (
            Android,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
            "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"
        ),
        (
            Linux,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"
        ),
        (
            IOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/134.0.0.0"
        )
    ]
);

mod_generator!(
    v135,
    v132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Google Chrome";v="135""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Google Chrome";v="135""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Google Chrome";v="135""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Google Chrome";v="135""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Google Chrome";v="135""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/135.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v136,
    v132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="136", "Not:A-Brand";v="24", "Google Chrome";v="136""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="136", "Not:A-Brand";v="24", "Google Chrome";v="136""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="136", "Not:A-Brand";v="24", "Google Chrome";v="136""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="136", "Not:A-Brand";v="24", "Google Chrome";v="136""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="136", "Not:A-Brand";v="24", "Google Chrome";v="136""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/136.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

mod_generator!(
    v137,
    v132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/137.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);
