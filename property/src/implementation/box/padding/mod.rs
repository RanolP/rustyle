
mod padding_bottom;
mod padding_left;
mod padding_right;
mod padding_top;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(padding_bottom::Instance);
    registerer.register(padding_left::Instance);
    registerer.register(padding_right::Instance);
    registerer.register(padding_top::Instance);
}
