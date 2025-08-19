const ACTION_ID: &str = "sign-up";
const ACTION_NAME: &str = "Sign up";

pub struct SignUpAction;

impl SignUpAction {
    pub fn id() -> String {
        ACTION_ID.to_owned()
    }

    pub fn name() -> String {
        ACTION_NAME.to_owned()
    }
}
