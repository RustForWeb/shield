use leptos::prelude::*;
use shield::ActionProviderForm;
use shield_leptos::Call;

use crate::leptos::input::FormInput;

#[component]
pub fn Form(action_id: String, form: ActionProviderForm) -> impl IntoView {
    let call = ServerAction::<Call>::new();

    view! {
        <ActionForm action=call>
            <input name="action_id" type="hidden" value=action_id />
            <input name="method_id" type="hidden" value=form.method_id />
            <input name="provider_id" type="hidden" value=form.provider_id />

            {form.form.inputs.into_iter().map(|input| view! {
                <FormInput input={input} />
            }).collect_view()}
        </ActionForm>
    }
}
