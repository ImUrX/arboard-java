use arboard::ImageData;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint, jlong};
use jni::JNIEnv;
use std::{borrow::Cow, panic};

#[no_mangle]
pub extern "system" fn Java_io_github_imurx_arboard_ImageData_imageDataNew(
    env: JNIEnv,
    _class: JClass,
    width: jint,
    height: jint,
    image: JByteArray,
) -> jlong {
    let image = env.convert_byte_array(image).unwrap();
    let image_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::from(image.as_slice()),
    };
    Box::into_raw(Box::new(image_data.to_owned_img())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_imurx_arboard_ImageData_imageDataGetBytes(
    env: JNIEnv,
    _class: JClass,
    image_ptr: jlong,
) -> jbyteArray {
    let image = &mut *(image_ptr as *mut ImageData);
    env.byte_array_from_slice(&image.bytes)
        .expect("Couldn't make ByteArray")
        .as_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_imurx_arboard_ImageData_imageDataDrop(
    _env: JNIEnv,
    _class: JClass,
    image_ptr: jlong,
) {
    panic::catch_unwind(|| {
        let _ = Box::from_raw(image_ptr as *mut ImageData);
    })
    .unwrap()
}
