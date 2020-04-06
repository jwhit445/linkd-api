use warp::Filter;
use serde_derive::{Deserialize, Serialize};
use hyper::Client;
use hyper_tls::HttpsConnector;
use warp::http::Uri;
use std::convert::Infallible;

const CLIENT_ID: &str = "1054012033008.1047331901381";
const CLIENT_SECRET: &str = "bab4c2d7d1a64490802a03caa28d3cbe";

const SLACK_SIGNIN_HTML: &str = "<html>
<head></head>
<body>
    <a href=\"https://slack.com/oauth/authorize?scope=users.profile:write&client_id=1054012033008.1047331901381&redirect_uri=http://44.230.123.63:8000/callback/slack\">
        <img alt=\"\"Sign in with Slack\"\" height=\"40\" width=\"172\" src=\"https://platform.slack-edge.com/img/sign_in_with_slack.png\" 
            srcset=\"https://platform.slack-edge.com/img/sign_in_with_slack.png 1x, https://platform.slack-edge.com/img/sign_in_with_slack@2x.png 2x\" />
    </a>
</body>
</html>";

#[tokio::main]
async fn main() {
    //GET /slack/signin
    let slack = warp::path!("slack" / "signin").map(|| {
        warp::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(SLACK_SIGNIN_HTML)
    });
    // GET /callback/{app}
    let callback = warp::path::param()
    .and(warp::query())
    .and_then(get_auth_token);
    println!("Running at 0.0.0.0:8000");
    let routes = warp::get().and(callback.or(slack));
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}

async fn get_auth_token(_: String,slack: SlackCallback) -> Result<impl warp::Reply, Infallible> {
    let url_str=format!("https://slack.com/api/oauth.access?client_id={}&client_secret={}&code={}",CLIENT_ID,CLIENT_SECRET,slack.code);
    println!("url for request: {}",url_str);
    let url=url_str.parse::<Uri>().expect("failed to parse URL");
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let res = client.get(url).await;
    match res {
        Ok(value) => {
            //TODO: deserialize value into struct and save fields that we care about into db
            Ok(value)
        },
        Err(error) => {
            println!("Unexpected error retrieving auth token:\n{}",error);
            return Ok(warp::http::Response::builder()
                .status(500)
                .body(hyper::body::Body::empty()).unwrap());
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SlackCallback {
    code: String,
    state: String,
}