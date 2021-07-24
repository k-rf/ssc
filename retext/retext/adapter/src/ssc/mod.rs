mod request_parser;

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
            let request_parser::ParsedText {
                highlight,
                color,
                message,
                separator
            } = request_parser::text_parse(req.text);

            let input_data = app::InputData {
                highlight,
                color,
                message,
                separator,
            };

            Ok(Response {
                value: app::highlight_message(input_data).highlighted_message,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request_factory(s: &str) -> Request {
        Request {
            text: String::from(s),
            api_app_id: String::new(),
            channel_id: String::new(),
            channel_name: String::new(),
            command: String::new(),
            is_enterprise_install: false,
            response_url: String::new(),
            team_domain: String::new(),
            team_id: String::new(),
            token: String::new(),
            trigger_id: String::new(),
            user_id: String::new(),
            user_name: String::new(),
        }
    }

    #[test]
    // text の第 1 引数のみ指定する
    fn test_post_text_only_first_arg() {
        let req = request_factory("a");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-white-a:");
    }

    #[test]
    // text の第 2 引数に white を指定する
    fn test_post_text_second_arg_white() {
        let req = request_factory("a white red");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-white-a: red");
    }

    #[test]
    // text の第 2 引数に yellow を指定する
    fn test_post_text_second_arg_yellow() {
        let req = request_factory("a yellow red");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-yellow-a: red");
    }

    #[test]
    // text の第 2 引数に white / yellow 以外を指定する
    fn test_post() {
        let req = request_factory("a blue red");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-white-a: blue red");
    }

    #[test]
    // text に改行がある
    fn test_post_text_contains_linefeed() {
        let req = request_factory("a yellow\nred");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-yellow-a:\nred");
    }

    #[test]
    // text の第 2 引数に当たる部分の手前に改行がある
    fn test_post_text_contains_linefeed_in_second_arg() {
        let req = request_factory("a\nyellow\nred");

        let v = if let Ok(res) = text::highlight::post(req) {
            res.value
        } else {
            String::new()
        };

        assert_eq!(v, ":alphabet-white-a:\nyellow\nred");
    }
}
