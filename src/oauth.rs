use oauth2::{
    basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType}, reqwest::async_http_client, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, ExtraTokenFields, RedirectUrl, Scope, StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OAuthScope {
    #[serde(rename = "ads:read")]
    AdsRead,
    #[serde(rename = "ads:write")]
    AdsWrite,
    #[serde(rename = "billing:read")]
    BillingRead,
    #[serde(rename = "billing:write")]
    BillingWrite,
    #[serde(rename = "biz_access:read")]
    BizAccessRead,
    #[serde(rename = "biz_access:write")]
    BizAccessWrite,
    #[serde(rename = "boards:read")]
    BoardsRead,
    #[serde(rename = "boards:read_secret")]
    BoardsReadSecret,
    #[serde(rename = "boards:write")]
    BoardsWrite,
    #[serde(rename = "boards:write_secret")]
    BoardsWriteSecret,
    #[serde(rename = "catalogs:read")]
    CatalogsRead,
    #[serde(rename = "catalogs:write")]
    CatalogsWrite,
    #[serde(rename = "pins:read")]
    PinsRead,
    #[serde(rename = "pins:read_secret")]
    PinsReadSecret,
    #[serde(rename = "pins:write")]
    PinsWrite,
    #[serde(rename = "pins:write_secret")]
    PinsWriteSecret,
    #[serde(rename = "user_accounts:read")]
    UserAccountsRead,
    #[serde(rename = "user_accounts:write")]
    UserAccountsWrite,
}

impl OAuthScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AdsRead,
            Self::AdsWrite,
            Self::BillingRead,
            Self::BillingWrite,
            Self::BizAccessRead,
            Self::BizAccessWrite,
            Self::BoardsRead,
            Self::BoardsReadSecret,
            Self::BoardsWrite,
            Self::BoardsWriteSecret,
            Self::CatalogsRead,
            Self::CatalogsWrite,
            Self::PinsRead,
            Self::PinsReadSecret,
            Self::PinsWrite,
            Self::PinsWriteSecret,
            Self::UserAccountsRead,
            Self::UserAccountsWrite,
        ]
    }
}

impl std::fmt::Display for OAuthScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AdsRead => write!(f, "ads:read"),
            Self::AdsWrite => write!(f, "ads:write"),
            Self::BillingRead => write!(f, "billing:read"),
            Self::BillingWrite => write!(f, "billing:write"),
            Self::BizAccessRead => write!(f, "biz_access:read"),
            Self::BizAccessWrite => write!(f, "biz_access:write"),
            Self::BoardsRead => write!(f, "boards:read"),
            Self::BoardsReadSecret => write!(f, "boards:read_secret"),
            Self::BoardsWrite => write!(f, "boards:write"),
            Self::BoardsWriteSecret => write!(f, "boards:write_secret"),
            Self::CatalogsRead => write!(f, "catalogs:read"),
            Self::CatalogsWrite => write!(f, "catalogs:write"),
            Self::PinsRead => write!(f, "pins:read"),
            Self::PinsReadSecret => write!(f, "pins:read_secret"),
            Self::PinsWrite => write!(f, "pins:write"),
            Self::PinsWriteSecret => write!(f, "pins:write_secret"),
            Self::UserAccountsRead => write!(f, "user_accounts:read"),
            Self::UserAccountsWrite => write!(f, "user_accounts:write"),
        }
    }
}

const AUTH_URL: &str = "https://www.pinterest.com/oauth/";
const TOKEN_URL: &str = "https://api.pinterest.com/v5/oauth/token";

#[derive(Debug, Clone)]
pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Clone, Default)]
pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub extra: InnerExtraTokenFields,
    pub expires_in: Option<Duration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InnerExtraTokenFields {
    pub response_type: String,
    pub refresh_token_expires_in: Option<u64>,
    pub refresh_token_expires_at: Option<u64>,
}
impl ExtraTokenFields for InnerExtraTokenFields {}

pub struct Oauth {
    basic_client: Client<
        BasicErrorResponse,
        StandardTokenResponse<InnerExtraTokenFields, BasicTokenType>,
        BasicTokenType,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
    >,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
}

impl Oauth {
    pub fn new(
        api_key_code: &str,
        api_secret_code: &str,
        callback_url: &str,
        scopes: Vec<OAuthScope>,
    ) -> Result<Self, Error> {
        let basic_client = Client::new(
            ClientId::new(api_key_code.to_owned()),
            Some(ClientSecret::new(api_secret_code.to_owned())),
            AuthUrl::new(AUTH_URL.to_owned())?,
            Some(TokenUrl::new(TOKEN_URL.to_owned())?),
        );
        let redirect_url = RedirectUrl::new(callback_url.to_string())?;
        let scopes: Vec<Scope> = scopes
            .into_iter()
            .map(|it| Scope::new(it.to_string()))
            .collect();
        Ok(Self {
            basic_client,
            redirect_url,
            scopes,
        })
    }

    pub fn oauth_url(&self, state: Option<String>) -> OAuthUrlResult {
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let csrf_token = match state {
            Some(ref state_value) => CsrfToken::new(state_value.clone()),
            None => CsrfToken::new_random(),
        };
        let (auth_url, _csrf_token) = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .authorize_url(|| csrf_token)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        OAuthUrlResult {
            oauth_url: auth_url.to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    pub async fn token(
        &self,
        pkce_verifier_str: &str,
        code: &str,
    ) -> Result<TokenResult, Error> {
        let pkce_verifier = oauth2::PkceCodeVerifier::new(pkce_verifier_str.to_owned());

        let token = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|e| Error::Oauth(format!("{:?}", e)))?;
        Ok(TokenResult {
            access_token: token.access_token().secret().to_string(),
            token_type: token.token_type().as_ref().to_string(),
            refresh_token: token.refresh_token().map(|it| it.secret().to_string()),
            expires_in: token.expires_in(),
            extra: token.extra_fields().clone()
        })
    }
}
