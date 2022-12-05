use napi::{CallContext, Error, JsObject, bindgen_prelude::Array, JsUnknown, JsString};
use napi_derive::{js_function};

#[js_function(3)]
fn set_password(ctx: CallContext) -> napi::Result<JsString> {
    //let (deferred, promise) = ctx.env.create_deferred()?;

    //let service = ctx.get::<String>(0)?;
    let username = ctx.get::<String>(1)?;
    //let password = ctx.get::<String>(2)?;

    Ok(ctx.env.create_string(username.as_str())?)
}

fn main() {
    println!("Hello, world!");
}
