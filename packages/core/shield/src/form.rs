use std::collections::HashMap;

/// HTML [attribute](https://html.spec.whatwg.org/multipage/syntax.html#attributes-2).
#[derive(Clone, Debug)]
pub enum Attribute {
    Boolean(bool),
    String(String),
}

/// HTML [form](https://html.spec.whatwg.org/multipage/forms.html#the-form-element).
#[derive(Clone, Debug)]
pub struct Form {
    pub inputs: Vec<Input>,
    pub attributes: Option<HashMap<String, Attribute>>,
}

/// HTML [input](https://html.spec.whatwg.org/multipage/input.html#the-input-element).
#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub r#type: InputType,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, Attribute>>,
}

/// HTML input [type](https://html.spec.whatwg.org/multipage/input.html#attr-input-type) and [attributes](https://html.spec.whatwg.org/multipage/input.html#input-type-attr-summary).
#[derive(Clone, Debug)]
pub enum InputType {
    Button {
        popovertarget: Option<String>,
        popovertargetaction: Option<String>,
    },
    Checkbox {
        checked: Option<bool>,
        required: Option<bool>,
    },
    Color {
        alpha: Option<bool>,
        autocomplete: Option<String>,
        colorspace: Option<String>,
        list: Option<String>,
    },
    Date {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
    DatetimeLocal {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
    Email {
        autocomplete: Option<String>,
        dirname: Option<String>,
        list: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        multiple: Option<bool>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    File {
        accept: Option<String>,
        multiple: Option<bool>,
        required: Option<bool>,
    },
    Hidden {
        autocomplete: Option<String>,
        dirname: Option<String>,
        required: Option<bool>,
    },
    Image {
        alt: Option<String>,
        formaction: Option<String>,
        formenctype: Option<String>,
        formmethod: Option<String>,
        formnovalidate: Option<bool>,
        formtarget: Option<String>,
        height: Option<String>,
        popovertarget: Option<String>,
        popovertargetaction: Option<String>,
        src: Option<String>,
        width: Option<String>,
    },
    Month {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
    Number {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
    Password {
        autocomplete: Option<String>,
        dirname: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    Radio {
        checked: Option<bool>,
        required: Option<bool>,
    },
    Range {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        step: Option<String>,
    },
    Reset {
        popovertarget: Option<String>,
        popovertargetaction: Option<String>,
    },
    Search {
        autocomplete: Option<String>,
        dirname: Option<String>,
        list: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    Submit {
        dirname: Option<String>,
        formaction: Option<String>,
        formenctype: Option<String>,
        formmethod: Option<String>,
        formnovalidate: Option<bool>,
        formtarget: Option<String>,
        popovertarget: Option<String>,
        popovertargetaction: Option<String>,
    },
    Tel {
        autocomplete: Option<String>,
        dirname: Option<String>,
        list: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    Text {
        autocomplete: Option<String>,
        dirname: Option<String>,
        list: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    Time {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
    Url {
        autocomplete: Option<String>,
        dirname: Option<String>,
        list: Option<String>,
        maxlength: Option<String>,
        minlength: Option<String>,
        pattern: Option<String>,
        placeholder: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        size: Option<String>,
    },
    Week {
        autocomplete: Option<String>,
        list: Option<String>,
        max: Option<String>,
        min: Option<String>,
        readonly: Option<bool>,
        required: Option<bool>,
        step: Option<String>,
    },
}
