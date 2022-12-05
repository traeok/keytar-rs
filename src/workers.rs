use napi::{Task};

struct SetPassword {
    service: String,
    account: String,
    password: String,
}

impl Task for SetPassword {
    //type Output = ;
    //type JsValue = ;

    fn compute(&mut self) {

    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {

    }

    fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {

    }
}