
///On success, will set jwt access token cookie and redirect to console
pub async fn index(req: Request<Body>) -> Markup {
    let host = req.uri().to_string();
    let title = "axum-htmx-signup";
    let desc = "Create an account";
    
    let content = html! {
        h1 align="center" { "Login" }
        form hx-ext="response-targets" hx-post="/validate-username" hx-swap="outerHTML" {(TARGET_ERROR)} data-loading-states {
            label for="username" { "Username" }
            input type="text" name="username" placeholder="username" required;
            small { "Unique & Permanent" }
            button data-loading-aria-busy { "Check Username" }
            #error {}
        }
    };

    page::page(&host, title, desc, content)
}