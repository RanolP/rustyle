
mod margin_bottom;
mod margin_left;
mod margin_right;
mod margin_top;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(margin_top::Instance);
    registerer.register(margin_bottom::Instance);
    registerer.register(margin_left::Instance);
    registerer.register(margin_right::Instance);
}
