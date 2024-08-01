use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::collections::HashMap;
use pinterest_api::{api::get_boards, oauth::{OAuthScope, Oauth}};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use url::Url;

pub const PKCE_VERIFIER: &str = "pkce_verifier";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/oauth", get(oauth))
        .route("/", get(root))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn oauth_client() -> Oauth {
    Oauth::new(
        &std::env::var("CLIENT_KEY").unwrap(),
        &std::env::var("CLIENT_SECRET").unwrap(),
        &std::env::var("CALLBACK_URL").unwrap(),
        OAuthScope::all(),
    ).unwrap()
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let res = oauth.oauth_url(None);
    cookies.add(Cookie::new(PKCE_VERIFIER, res.pkce_verifier.clone()));
    Html(format!("<a href='{}'>oauth<a>", res.oauth_url)).into_response()
}

async fn oauth(uri: Uri, cookies: Cookies) -> impl IntoResponse {
    let url = Url::parse(&format!("http://localhost:3000{}", uri)).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let pkce = cookies.get(PKCE_VERIFIER).unwrap();
    let oauth = oauth_client();
    let res = oauth.token(pkce.value(), hash_query.get("code").unwrap()).await.unwrap();
    println!("{:?}", res);
    let result = get_boards::Api::new(None)
        .execute(&res.access_token)
        .await;
    println!("{:?}", result);
    "success".into_response()
}
