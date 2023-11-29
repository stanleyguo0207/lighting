// mod test_iteration;
// mod test_progress;
// mod ui;
// mod test_forminput;
mod test_children;

use leptos::*;
// use test_forminput::*;
// use test_iteration::*;
// use test_progress::*;
// use ui::*;
use test_children::*;

fn main() {
  // mount_to_body(Ui)
  // mount_to_body(TestProgress)
  // mount_to_body(TestIteration)
  // mount_to_body(TestFormInput)
  mount_to_body(TestChildren)
}
