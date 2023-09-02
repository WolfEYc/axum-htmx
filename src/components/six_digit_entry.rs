use maud::{Markup, html};

pub fn six_digit_entry() -> Markup {
    html! {
        label class="center" for="six_digits" { "Enter your 6 digit auth code" }
        input name="six_digits" id="six_digits" class="center" type="text" pattern=r"\d*" maxlength="6" style="width:6rem" placeholder="XXXXXX" required;
    }
}