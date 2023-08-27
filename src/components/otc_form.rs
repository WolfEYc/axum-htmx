use maud::{Markup, html};

pub fn otc_form() -> Markup {
    html! {
    head {
        link rel="stylesheet" href="css/otc-form.min.css";
    }
    div class="otc" name="one-time-code" {
        fieldset {
            legend { "Validation Code" }
            label for="otc-1" { "Number 1" }
            label for="otc-2" { "Number 2" }
            label for="otc-3" { "Number 3" }
            label for="otc-4" { "Number 4" }
            label for="otc-5" { "Number 5" }
            label for="otc-6" { "Number 6" }

            div {
                input type="number" pattern="[0-9]*"  value="" inputtype="numeric" autocomplete="one-time-code" id="otc-1" required;
                input type="number" pattern="[0-9]*" min="0" max="9" maxlength="1"  value="" inputtype="numeric" id="otc-2" required;
                input type="number" pattern="[0-9]*" min="0" max="9" maxlength="1"  value="" inputtype="numeric" id="otc-3" required;
                input type="number" pattern="[0-9]*" min="0" max="9" maxlength="1"  value="" inputtype="numeric" id="otc-4" required;
                input type="number" pattern="[0-9]*" min="0" max="9" maxlength="1"  value="" inputtype="numeric" id="otc-5" required;
                input type="number" pattern="[0-9]*" min="0" max="9" maxlength="1"  value="" inputtype="numeric" id="otc-6" required;
            }
        }
    }
    script src="js/otp-form.min.js" {}
    }
}