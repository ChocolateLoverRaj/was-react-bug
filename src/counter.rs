use wasm_react::{
    clones, h,
    hooks::{use_effect, use_state, Deps},
    Component, VNode,
};

pub struct Counter;
impl Component for Counter {
    fn render(&self) -> VNode {
        let counter = use_state(|| false);

        use_effect(
            {
                clones!(mut counter);
                counter.set(|mut _counter| true);

                move || || ()
            },
            Deps::none(),
        );

        h!(div).build("<Component />")
    }
}
