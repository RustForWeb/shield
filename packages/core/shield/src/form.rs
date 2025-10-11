use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub inputs: Vec<Input>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub name: String,
    pub label: Option<String>,
    pub r#type: InputType,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "kebab-case")]
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

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Button(_) => "button",
            InputType::Checkbox(_) => "checkbox",
            InputType::Color(_) => "color",
            InputType::Date(_) => "date",
            InputType::DatetimeLocal(_) => "datetime-local",
            InputType::Email(_) => "email",
            InputType::File(_) => "file",
            InputType::Hidden(_) => "hidden",
            InputType::Image(_) => "image",
            InputType::Month(_) => "month",
            InputType::Number(_) => "number",
            InputType::Password(_) => "password",
            InputType::Radio(_) => "radio",
            InputType::Range(_) => "range",
            InputType::Reset(_) => "reset",
            InputType::Search(_) => "search",
            InputType::Submit(_) => "submit",
            InputType::Tel(_) => "tel",
            InputType::Text(_) => "text",
            InputType::Time(_) => "time",
            InputType::Url(_) => "url",
            InputType::Week(_) => "week",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeButton {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeCheckbox {
    pub checked: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeColor {
    pub alpha: Option<bool>,
    pub autocomplete: Option<String>,
    pub colorspace: Option<String>,
    pub list: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeDate {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeDatetimeLocal {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeEmail {
    pub autocomplete: Option<String>,
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeFile {
    pub accept: Option<String>,
    pub multiple: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeHidden {
    pub autocomplete: Option<String>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeImage {
    pub alt: Option<String>,
    pub height: Option<String>,
    pub src: Option<String>,
    pub width: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeMonth {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypePassword {
    pub autocomplete: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeRadio {
    pub checked: Option<bool>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeRange {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeReset {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeSearch {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeSubmit {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeTel {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeText {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeTime {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeUrl {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InputTypeWeek {
    pub autocomplete: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub step: Option<String>,
}
