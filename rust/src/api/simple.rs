use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyOpaqueType {
    message: String,
}

pub fn create_opaque(message: String) -> MyOpaqueType {
    MyOpaqueType { message }
}

pub async fn use_opaque(object: Option<&MyOpaqueType>) -> String {
    format!("Message: {}", object.unwrap().message)
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
