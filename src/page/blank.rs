use super::ViewPage;
use seed::*;

// ------ ------
//     View
// ------ ------

pub fn view<'a, Ms>() -> ViewPage<'a, Ms> {
    ViewPage::new("Blank", empty!())
}
