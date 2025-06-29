const ACTION_ID: &str = "sign-in";
const ACTION_NAME: &str = "Sign in";

pub struct SignInAction;

impl SignInAction {
    pub fn id() -> String {
        ACTION_ID.to_owned()
    }

    pub fn name() -> String {
        ACTION_NAME.to_owned()
    }
}
