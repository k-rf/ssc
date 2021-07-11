use application as app;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Request {
    text: String,
    api_app_id: String,
    channel_id: String,
    channel_name: String,
    command: String,
    is_enterprise_install: bool,
    response_url: String,
    team_domain: String,
    team_id: String,
    token: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
}

pub mod text {
    use super::*;

    pub mod highlight {
        use super::*;

        pub struct Response {
            pub value: String,
        }

        pub fn post(req: Request) -> Result<Response, std::io::Error> {
            let mut s = req.text.split_whitespace();

            let highlight = String::from(s.next().expect("Highlight text is necessary"));
            let color = s.next();
            let message = s.collect::<Vec<_>>().join(" ");

            let input_data = app::InputData {
                highlight,
                color,
                message: Some(message.as_str()),
            };

            Ok(Response {
                value: app::highlight_message(input_data).highlighted_message,
            })
        }
    }
}
