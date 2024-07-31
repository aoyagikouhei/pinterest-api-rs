use std::time::Duration;

use reqwest::RequestBuilder;

const URL_PREFIX: &str = "https://api.pinterest.com/v5";
const ENV_KEY: &str = "PINTEREST_PREFIX_API";

#[derive(Debug, Clone, Default)]
pub struct ApiOptions {
    pub prefix_url: Option<String>,
    pub timeout: Option<Duration>,
}

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, URL_PREFIX);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url(postfix_url: &str, options: &Option<ApiOptions>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(URL_PREFIX.to_owned()),
        options,
        postfix_url,
    )
}

fn make_url_with_prefix(
    default_perfix_url: &str,
    options: &Option<ApiOptions>,
    postfix_url: &str,
) -> String {
    let prefix_url = if let Some(options) = options {
        if let Some(prefix_url) = options.prefix_url.as_ref() {
            prefix_url
        } else {
            default_perfix_url
        }
    } else {
        default_perfix_url
    };
    format!("{}{}", prefix_url, postfix_url)
}

pub(crate) fn apply_options(
    builder: RequestBuilder,
    options: &Option<ApiOptions>,
) -> RequestBuilder {
    if let Some(options) = options {
        if let Some(timeout) = options.timeout {
            builder.timeout(timeout)
        } else {
            builder
        }
    } else {
        builder
    }
}
