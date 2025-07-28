use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button
            on:click=move |_| {
                *set_count.write() +=10;
            }
            style="position: absolute"
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({},{}, 100)", count.get(), 100)
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >
        "Click to move"
        </button>
        <progress
            max="50"
            value=count
        />
    }
}
