use arboard::{Clipboard, ImageData};
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use std::panic;

#[no_mangle]
pub extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let clipboard = Clipboard::new().unwrap();
    Box::into_raw(Box::new(clipboard)) as jlong
}

pub unsafe extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardGetText(
    env: JNIEnv,
    _class: JClass,
    clipboard_ptr: jlong,
) -> jstring {
    let clipboard = &mut *(clipboard_ptr as *mut Clipboard);
    env.new_string(clipboard.get_text().unwrap())
        .expect("Java string couldn't be made")
        .into_inner()
}

pub unsafe extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardSetText(
    env: JNIEnv,
    _class: JClass,
    clipboard_ptr: jlong,
    input: JString,
) {
    let clipboard = &mut *(clipboard_ptr as *mut Clipboard);
    let input: String = env
        .get_string(input)
        .expect("Couldn't get Java string")
        .into();
    clipboard.set_text(input).unwrap()
}

pub unsafe extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardGetImage(
    _env: JNIEnv,
    _class: JClass,
    clipboard_ptr: jlong,
) -> jlong {
    let clipboard = &mut *(clipboard_ptr as *mut Clipboard);
    let image = clipboard.get_image().unwrap();
    Box::into_raw(Box::new(image)) as jlong
}

pub unsafe extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardSetImage(
    _env: JNIEnv,
    _class: JClass,
    clipboard_ptr: jlong,
    image_ptr: jlong,
) {
    let clipboard = &mut *(clipboard_ptr as *mut Clipboard);
    let image = &mut *(image_ptr as *mut ImageData);
    clipboard.set_image(image.clone()).unwrap();
}

pub unsafe extern "system" fn Java_io_github_imurx_arboard_Clipboard_clipboardDrop(
    _env: JNIEnv,
    _class: JClass,
    clipboard_ptr: jlong,
) {
    panic::catch_unwind(|| {
        Box::from_raw(clipboard_ptr as *mut Clipboard);
    })
    .unwrap()
}
