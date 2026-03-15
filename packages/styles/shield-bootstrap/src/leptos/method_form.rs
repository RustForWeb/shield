use leptos::prelude::*;
use shield_leptos::CallMethod;

use crate::leptos::input::FormInput;

#[component]
pub fn MethodForm(
    action_id: String,
    method_id: String,
    provider_id: Option<String>,
    form: shield::Form,
) -> impl IntoView {
    let call_method = ServerAction::<CallMethod>::new();

    view! {
        <ActionForm action=call_method>
            <input name="action_id" type="hidden" value=action_id />
            <input name="method_id" type="hidden" value=method_id />
            <input name="provider_id" type="hidden" value=provider_id />

            {form.inputs.into_iter().map(|input| view! {
                <FormInput input={input} />
            }).collect_view()}
        </ActionForm>
    }
}
