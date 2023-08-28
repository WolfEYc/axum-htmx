use maud::{html, Markup};

pub fn copyblock(name: String, text: String) -> Markup {
    html! {
        copyblock class="copyblock" {
            input type="text" name=(name) value=(text) readonly;
        }
    }
}