use wasm_bindgen::JsValue;
use wasm_react::hooks::{use_effect, use_js_ref, Deps};
use wasm_react::props::Props;
use wasm_react::{clones, create_element, export_components, Component, VNode};
mod counter;

use wasm_react::{create_context, hooks::State, Context};
use web_sys::HtmlVideoElement;

thread_local! {
  pub static STATE_CONTEXT: Context<Option<State<i32>>> = create_context(None.into());
}

pub struct App;

impl Component for App {
    fn render(&self) -> VNode {
        let video_ref = use_js_ref(None::<HtmlVideoElement>);

        use_effect(
            {
                clones!(video_ref);
                move || {
                    let video = video_ref.current().unwrap();
                    video.set_src("something".into());
                }
            },
            Deps::none(),
        );

        create_element(
            &"video".into(),
            &Props::new().ref_container(&video_ref),
            ().into(),
        )
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }
