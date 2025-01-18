
fn hello_from_rust(name:&str) -> String {
    format!("Hello {name} from Rust!")
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_uk_co_davehadley_exampleandroidrust_ExampleRustLibrary_helloFromRust(mut env: JNIEnv, _: JClass, name: JString) -> jstring {
        let name: String = env.get_string(&name).expect("failed to get name string").into();
        let output = hello_from_rust(name.as_str());
        let output = env.new_string(output).expect("failed to convert output to Java string");
        output.into_raw()
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    type Result = std::result::Result<(), Box<dyn Error>>;

    #[test]
    fn test_rust_greeting() -> Result {
        let name = "Dave";
        let result = hello_from_rust(name);
        assert_eq!(result, "Hello Dave from Rust!");
        Ok(())
    }
}