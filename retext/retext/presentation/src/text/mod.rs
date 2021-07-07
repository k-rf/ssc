use application as app;

pub struct Request {
    pub text: String,
    pub color: String,
}

pub struct Response {
    pub value: String,
}

pub fn convert(req: Request) -> Response {
    let input_data = app::InputData {
        raw_text: req.text,
        color: match req.color.as_str() {
            "white" => app::Color::White,
            "yellow" => app::Color::Yellow,
            _ => app::Color::White,
        },
    };

    Response {
        value: app::raw2emoji(input_data).emoji_text,
    }
}
