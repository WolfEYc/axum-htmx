use maud::{html, Markup};

pub fn copyblock(text: String) -> Markup {
    html! {
        button type="button" id="cpybtn" class="secondary outline center" data-tooltip="Copy" onclick=(format!("copyToClipboard('{}', this)", text)) { (text) }
    }
}
