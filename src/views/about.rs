use dioxus::prelude::*;

use crate::components::{Echo, Hero};

#[component]
pub fn About() -> Element {
    rsx! {
        Hero {  }
        Echo {  }
    }
}
