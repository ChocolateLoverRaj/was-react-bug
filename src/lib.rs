use wasm_bindgen::JsValue;
use wasm_react::hooks::{use_context, use_state};
use wasm_react::{clones, export_components, h, Callback, Component, ContextProvider, VNode};
mod counter;

use wasm_react::{create_context, hooks::State, Context};

thread_local! {
  pub static STATE_CONTEXT: Context<Option<State<i32>>> = create_context(None.into());
}

pub struct App;

impl Component for App {
    fn render(&self) -> VNode {
        let state = use_state(|| 0);
        ContextProvider::from(&STATE_CONTEXT)
            .value(Some(Some(state).into()))
            .build(SubComponent {}.build())
    }
}

pub struct SubComponent;

impl Component for SubComponent {
    fn render(&self) -> VNode {
        let context = use_context(&STATE_CONTEXT);
        let v_node: VNode = match context.as_ref() {
            Some(state) => (*state.value()).into(),
            None => "No context provider".into(),
        };
        h!(div).build((
            h!(h1).build(v_node),
            h!(button)
                .on_click(&Callback::new({
                    move |_| {
                        if let Some(state) = context.as_ref() {
                            clones!(mut state);
                            state.set(|a| a + 1);
                        }
                    }
                }))
                .build("Change state of higher component"),
        ))
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }
