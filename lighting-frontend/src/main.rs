// mod test_iteration;
// mod test_progress;
// mod ui;
// mod test_forminput;
// mod test_children;
// mod test_effect;
// mod test_asyncload;
mod test_suspense;

use leptos::*;
// use test_forminput::*;
// use test_iteration::*;
// use test_progress::*;
// use ui::*;
// use test_children::*;
// use test_effect::*;
// use test_asyncload::*;
use test_suspense::*;

fn main() {
  // mount_to_body(Ui)
  // mount_to_body(TestProgress)
  // mount_to_body(TestIteration)
  // mount_to_body(TestFormInput)
  // mount_to_body(TestChildren)
  // mount_to_body(TestEffect)
  // mount_to_body(TestAsyncLoad)
  mount_to_body(TestSuspense)
}
