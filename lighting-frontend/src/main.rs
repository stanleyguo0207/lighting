// mod test_iteration;
// mod test_progress;
// mod ui;
mod test_forminput;

use leptos::*;
use test_forminput::*;
// use test_iteration::*;
// use test_progress::*;
// use ui::*;

fn main() {
  // mount_to_body(Ui)
  // mount_to_body(TestProgress)
  // mount_to_body(TestIteration)
  mount_to_body(TestFormInput)
}
