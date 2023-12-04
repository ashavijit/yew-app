use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[function_component]
fn MyComponent() -> Html {
    html! {
        <div>
            <p>{ "Hello World!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    yew::Renderer::<MyComponent>::new().render();
}