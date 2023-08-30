use crate::strings;
use axum::http::HeaderMap;
use maud::{html, Markup, DOCTYPE};

fn body(content: Markup) -> Markup {
    html! {
        body hx-ext="loading-states" {
            main class="container" {
                (content)
            }
            script src="js/htmx.min.js" {}
            script src="js/loading-states.min.js" {}
            script src="js/response-targets.min.js" {}
            script src="js/copytext.js" {}
            // TODO: Google Analytics: change UA-XXXXX-Y to be your site's ID.
            (google_analytics("UA-XXXXX-Y"))
            // Non-H5BP editorial comment: please consider using another analytics solution
            // instead of gifting your users' data to Alphabet Inc. - see e.g.
            // <https://mentalpivot.com/ethical-web-analytics-alternatives-google/>
            // for a discussion of alternatives.
        }
    }
}

fn google_analytics(site_id: &str) -> Markup {
    html! {
        script {"
            window.ga = function () {{ ga.q.push(arguments) }}; ga.q = []; ga.l = +new Date;
            ga('create', '" (site_id) "', 'auto'); ga('set', 'anonymizeIp', true); ga('set', 'transport', 'beacon'); ga('send', 'pageview')" }
        script src="https://www.google-analytics.com/analytics.js" async {}
    }
}

fn head(title: &str, desc: &str, url: &str) -> Markup {
    html! {
        head {
            meta charset=(strings::UTF8);
            title { (title) }
            meta name=(strings::DESCRIPTION) content=(desc);
            meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
            meta property="og:title" content=(title);
            meta property="og:type" content=(strings::WEBSITE);
            meta property="og:url" content=(url);
            meta property="og:image" content="";
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="manifest" href="site.webmanifest";
            link rel="apple-touch-icon" href="icon.png";
            link rel="stylesheet" href="css/pico.min.css";
            link rel="stylesheet" href="css/pico_overrides.css";
            meta name="theme-color" content="#fafafa";
        }
    }
}

pub(crate) fn page(host: &str, title: &str, desc: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html class="no-js" lang="en" {
            (head(title, desc, host))
            (body(content))
        }
    }
}

pub fn hx_redirect(path: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", path.parse().unwrap());
    headers
}
