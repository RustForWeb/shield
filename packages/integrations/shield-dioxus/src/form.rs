use dioxus::prelude::*;
use shield::{Form, Input, InputType};

pub trait ToRsx {
    fn to_rsx(&self) -> Element;
}

impl ToRsx for Form {
    fn to_rsx(&self) -> Element {
        rsx! {
            form {
                {self.inputs.iter().map(ToRsx::to_rsx)}
            }
        }
    }
}

impl ToRsx for Input {
    fn to_rsx(&self) -> Element {
        let input = match &self.r#type {
            InputType::Button(button) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "button",
                    popovertarget: button.popovertarget.clone(),
                    popovertargetaction: button.popovertargetaction.clone(),
                }
            },
            InputType::Checkbox(checkbox) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "checkbox",
                    checked: checkbox.checked,
                    required: checkbox.required,
                }
            },
            InputType::Color(color) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "color",
                    "alpha": color.alpha,
                    autocomplete: color.autocomplete.clone(),
                    "colorspace": color.colorspace.clone(),
                    list: color.list.clone(),
                }
            },
            InputType::Date(date) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "date",
                    autocomplete: date.autocomplete.clone(),
                    list: date.list.clone(),
                    max: date.max.clone(),
                    min: date.min.clone(),
                    readonly: date.readonly,
                    required: date.required,
                    step: date.step.clone(),
                }
            },
            InputType::DatetimeLocal(datetime_local) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "datetime-local",
                    autocomplete: datetime_local.autocomplete.clone(),
                    list: datetime_local.list.clone(),
                    max: datetime_local.max.clone(),
                    min: datetime_local.min.clone(),
                    readonly: datetime_local.readonly,
                    required: datetime_local.required,
                    step: datetime_local.step.clone(),
                }
            },
            InputType::Email(email) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "email",
                    autocomplete: email.autocomplete.clone(),
                    "dirname": email.dirname.clone(),
                    list: email.list.clone(),
                    maxlength: email.maxlength.clone(),
                    minlength: email.minlength.clone(),
                    multiple: email.multiple,
                    pattern: email.pattern.clone(),
                    placeholder: email.placeholder.clone(),
                    readonly: email.readonly,
                    required: email.required,
                    size: email.size.clone(),
                }
            },
            InputType::File(file) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "file",
                    accept: file.accept.clone(),
                    multiple: file.multiple,
                    required: file.required,
                }
            },
            InputType::Hidden(hidden) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "hidden",
                    autocomplete: hidden.autocomplete.clone(),
                    "dirname": hidden.dirname.clone(),
                    required: hidden.required,
                }
            },
            InputType::Image(image) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "image",
                    alt: image.alt.clone(),
                    formaction: image.formaction.clone(),
                    formenctype: image.formenctype.clone(),
                    formmethod: image.formmethod.clone(),
                    formnovalidate: image.formnovalidate,
                    formtarget: image.formtarget.clone(),
                    height: image.height.clone(),
                    popovertarget: image.popovertarget.clone(),
                    popovertargetaction: image.popovertargetaction.clone(),
                    src: image.src.clone(),
                    width: image.width.clone(),
                }
            },
            InputType::Month(month) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "month",
                    autocomplete: month.autocomplete.clone(),
                    list: month.list.clone(),
                    max: month.max.clone(),
                    min: month.min.clone(),
                    readonly: month.readonly,
                    required: month.required,
                    step: month.step.clone(),
                }
            },
            InputType::Number(number) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "number",
                    autocomplete: number.autocomplete.clone(),
                    list: number.list.clone(),
                    max: number.max.clone(),
                    min: number.min.clone(),
                    placeholder: number.placeholder.clone(),
                    readonly: number.readonly,
                    required: number.required,
                    step: number.step.clone(),
                }
            },
            InputType::Password(password) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "password",
                    autocomplete: password.autocomplete.clone(),
                    "dirname": password.dirname.clone(),
                    maxlength: password.maxlength.clone(),
                    minlength: password.minlength.clone(),
                    pattern: password.pattern.clone(),
                    placeholder: password.placeholder.clone(),
                    readonly: password.readonly,
                    required: password.required,
                    size: password.size.clone(),
                }
            },
            InputType::Radio(radio) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "radio",
                    checked: radio.checked,
                    required: radio.required,
                }
            },
            InputType::Range(range) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "range",
                    autocomplete: range.autocomplete.clone(),
                    list: range.list.clone(),
                    max: range.max.clone(),
                    min: range.min.clone(),
                    step: range.step.clone(),
                }
            },
            InputType::Reset(reset) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "reset",
                    popovertarget: reset.popovertarget.clone(),
                    popovertargetaction: reset.popovertargetaction.clone(),
                }
            },
            InputType::Search(search) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "search",
                    autocomplete: search.autocomplete.clone(),
                    "dirname": search.dirname.clone(),
                    list: search.list.clone(),
                    maxlength: search.maxlength.clone(),
                    minlength: search.minlength.clone(),
                    pattern: search.pattern.clone(),
                    placeholder: search.placeholder.clone(),
                    readonly: search.readonly,
                    required: search.required,
                    size: search.size.clone(),
                }
            },
            InputType::Submit(submit) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "submit",
                    "dirname": submit.dirname.clone(),
                    formaction: submit.formaction.clone(),
                    formenctype: submit.formenctype.clone(),
                    formmethod: submit.formmethod.clone(),
                    formnovalidate: submit.formnovalidate,
                    formtarget: submit.formtarget.clone(),
                    popovertarget: submit.popovertarget.clone(),
                    popovertargetaction: submit.popovertargetaction.clone(),
                }
            },
            InputType::Tel(tel) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "tel",
                    autocomplete: tel.autocomplete.clone(),
                    "dirname": tel.dirname.clone(),
                    list: tel.list.clone(),
                    maxlength: tel.maxlength.clone(),
                    minlength: tel.minlength.clone(),
                    pattern: tel.pattern.clone(),
                    placeholder: tel.placeholder.clone(),
                    readonly: tel.readonly,
                    required: tel.required,
                    size: tel.size.clone(),
                }
            },
            InputType::Text(text) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "text",
                    autocomplete: text.autocomplete.clone(),
                    "dirname": text.dirname.clone(),
                    list: text.list.clone(),
                    maxlength: text.maxlength.clone(),
                    minlength: text.minlength.clone(),
                    pattern: text.pattern.clone(),
                    placeholder: text.placeholder.clone(),
                    readonly: text.readonly,
                    required: text.required,
                    size: text.size.clone(),
                }
            },
            InputType::Time(time) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "time",
                    autocomplete: time.autocomplete.clone(),
                    list: time.list.clone(),
                    max: time.max.clone(),
                    min: time.min.clone(),
                    readonly: time.readonly,
                    required: time.required,
                    step: time.step.clone(),
                }
            },
            InputType::Url(url) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "url",
                    autocomplete: url.autocomplete.clone(),
                    "dirname": url.dirname.clone(),
                    list: url.list.clone(),
                    maxlength: url.maxlength.clone(),
                    minlength: url.minlength.clone(),
                    pattern: url.pattern.clone(),
                    placeholder: url.placeholder.clone(),
                    readonly: url.readonly,
                    required: url.required,
                    size: url.size.clone(),
                }
            },
            InputType::Week(week) => rsx! {
                input {
                    name: self.name.clone(),
                    value: self.value.clone(),
                    r#type: "week",
                    autocomplete: week.autocomplete.clone(),
                    list: week.list.clone(),
                    max: week.max.clone(),
                    min: week.min.clone(),
                    readonly: week.readonly,
                    required: week.required,
                    step: week.step.clone(),
                }
            },
        };

        rsx! {
            div {
                if let Some(label) = &self.label {
                    label { "{label}" }
                }

                {input}
            }
        }
    }
}
