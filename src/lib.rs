#![allow(non_snake_case)]
use chua::upload_blocking;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;

const OK: i32 = 0;
const NULL: i32 = 1;
const INVALID: i32 = 2;
const UPLOAD: i32 = 3;

#[no_mangle]
pub extern "system" fn Java_com_live2o3_chua_Chua_upload(
    env: JNIEnv,
    _class: JClass,
    url: JString,
    path: JString,
    chunk_size: jint,
    parallel: jint,
) -> i32 {
    #[cfg(target_os = "android")]
    init_android_log();

    if url.is_null() || path.is_null() {
        return NULL;
    }

    if chunk_size < 0 || parallel < 0 {
        return INVALID;
    }

    let url: String = match env.get_string(url) {
        Ok(s) => s.into(),
        Err(_) => return INVALID,
    };

    let path: String = match env.get_string(path) {
        Ok(s) => s.into(),
        Err(_) => return INVALID,
    };

    match upload_blocking(&url, path, chunk_size as usize, parallel as usize) {
        Ok(()) => OK,
        Err(e) => {
            #[cfg(target_os = "android")]
            log::error!("Chua: {}", e);
            UPLOAD
        }
    }
}

#[cfg(target_os = "android")]
fn init_android_log() {
    use android_logger::{Config, FilterBuilder};
    use log::Level;
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("mytag") // logs will show under mytag tag
            .with_filter(
                // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );
}
