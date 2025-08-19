use leptos::prelude::*;
use shield::Input;

#[component]
pub fn FormInput(input: Input) -> impl IntoView {
    view! {
        <div class="mb-3">
            <Label label={input.label.clone()} />
            <Control input={input} />
        </div>
    }
}

#[component]
fn Label(label: Option<String>) -> impl IntoView {
    label.map(|label| {
        view! {
            <label class="form-label">{label.clone()}</label>
        }
    })
}

#[component]
fn Control(input: Input) -> impl IntoView {
    view! {
         <input
            class="form-control"
            // TODO: Support nested data (`data[user[name]]` should instead be `data[user][name]`).
            name=format!("data[{}]", input.name)
            r#type=input.r#type.as_str()
            value=input.value.clone()
        />
    }
}
