mod ui;

use leptos::*;
use ui::*;

fn main() {
  mount_to_body(|| {
    view! { <Ui/> }
  })
}
