use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// HTML [attribute](https://html.spec.whatwg.org/multipage/syntax.html#attributes-2).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Attribute {
    Boolean(bool),
    String(String),
}

/// HTML [form](https://html.spec.whatwg.org/multipage/forms.html#the-form-element).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Form {
    pub inputs: Vec<Input>,
    pub attributes: Option<HashMap<String, Attribute>>,
}

/// HTML [input](https://html.spec.whatwg.org/multipage/input.html#the-input-element).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    pub name: String,
    pub label: Option<String>,
    pub r#type: InputType,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, Attribute>>,
}

/// HTML input [type](https://html.spec.whatwg.org/multipage/input.html#attr-input-type) and [attributes](https://html.spec.whatwg.org/multipage/input.html#input-type-attr-summary).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InputType {
    Button(InputTypeButton),
    Checkbox(InputTypeCheckbox),
    Color(InputTypeColor),
    Date(InputTypeDate),
    DatetimeLocal(InputTypeDatetimeLocal),
    Email(InputTypeEmail),
    File(InputTypeFile),
    Hidden(InputTypeHidden),
    Image(InputTypeImage),
    Month(InputTypeMonth),
    Number(InputTypeNumber),
    Password(InputTypePassword),
    Radio(InputTypeRadio),
    Range(InputTypeRange),
    Reset(InputTypeReset),
    Search(InputTypeSearch),
    Submit(InputTypeSubmit),
    Tel(InputTypeTel),
    Text(InputTypeText),
    Time(InputTypeTime),
    Url(InputTypeUrl),
    Week(InputTypeWeek),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeButton {
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeCheckbox {
    pub checked: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeColor {
    pub alpha: Option<bool>,
    pub autocomplete: Option<String>,
    pub colorspace: Option<String>,
    pub list: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeDate {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeDatetimeLocal {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeEmail {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub multiple: Option<bool>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeFile {
    pub accept: Option<String>,
    pub multiple: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeHidden {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeImage {
    pub alt: Option<String>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: Option<bool>,
    pub formtarget: Option<String>,
    pub height: Option<String>,
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
    pub src: Option<String>,
    pub width: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeMonth {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeNumber {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypePassword {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeRadio {
    pub checked: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeRange {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeReset {
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeSearch {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeSubmit {
    pub dirname: Option<String>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: Option<bool>,
    pub formtarget: Option<String>,
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeTel {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeText {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeTime {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeUrl {
    pub autocomplete: Option<String>,
    pub dirname: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputTypeWeek {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}
