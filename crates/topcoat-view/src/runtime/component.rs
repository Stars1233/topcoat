use topcoat_core::context::Cx;

use crate::runtime::View;

pub trait Component {
    type Error;

    fn render(self, cx: &Cx, child: View)
    -> impl Future<Output = Result<View, Self::Error>> + Send;
}
