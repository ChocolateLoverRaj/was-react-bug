use wasm_react::{
    clones, h,
    hooks::{use_effect, use_state, Deps},
    Component, VNode,
};

pub struct Counter;
impl Component for Counter {
    fn render(&self) -> VNode {
        let counter = use_state(|| false);

        clones!(mut counter);
        use_effect(
            move || {
                counter.set(|mut _counter| true);
            },
            Deps::none(),
        );

        h!(div).build("<Component />")
    }
}
