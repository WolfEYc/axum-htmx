use maud::{html, PreEscaped, Markup, DOCTYPE};
use crate::strings;

pub async fn not_found() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset=(strings::UTF8);
                title { (strings::NOT_FOUND_TITLE) }
                meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                style { (strings::NOT_FOUND_STYLE) }
                link rel="stylesheet" href="css/pico.min.css";
            }
            body {
                h1 { (strings::NOT_FOUND_TITLE) }
                p { (strings::NOT_FOUND_MESSAGE) }
            }
            (PreEscaped(strings::NOT_FOUND_COMMENT))
        }
    }
}