use anyhow::Result;
use encoding::all::GBK;
use encoding::{EncoderTrap, Encoding};
use libloading;
use libloading::{Library, Symbol};
use std::ffi::{c_char, c_int, c_short, c_uint, c_void};

fn main() -> Result<()> {

    let lib: Library = unsafe { Library::new("lib/printer.sdk.dll".to_string()).unwrap() };

    let InitPrinter = unsafe {
        let test: Symbol<unsafe extern "C" fn(p_char: *const std::ffi::c_short) -> *const c_void> =
            lib.get("InitPrinter".as_ref())?;
        test
    };

    let ReleasePrinter = unsafe {
        let test: Symbol<unsafe extern "C" fn(v: *const std::ffi::c_void) -> c_int> =
            lib.get("ReleasePrinter".as_ref())?;
        test
    };

    let OpenPort = unsafe {
        let test: Symbol<
            unsafe extern "C" fn(
                hPrinter: *const std::ffi::c_void,
                setting: *const std::ffi::c_char,
            ) -> c_int,
        > = lib.get("OpenPort".as_ref())?;
        test
    };

    let FeedLine = unsafe {
        let test: Symbol<
            unsafe extern "C" fn(hPrinter: *const std::ffi::c_void, size: c_uint) -> c_int,
        > = lib.get("FeedLine".as_ref())?;
        test
    };

    let PrintText = unsafe {
        let test: Symbol<
            unsafe extern "C" fn(
                hPrinter: *const std::ffi::c_void,
                buffer: *const std::ffi::c_char,
                alignment: c_uint,
                size: c_uint,
            ) -> c_int,
        > = lib.get("PrintText".as_ref())?;
        test
    };

    unsafe {
        let string_value = "\0\0".to_string();
        let printer_result = InitPrinter(string_value.as_ptr() as *const c_short);

        println!("InitPrinter:{:?}", printer_result);

        let setting_value = "U\0S\0B\0,\0\0\0".to_string();
        let open_port_result = OpenPort(printer_result, setting_value.as_ptr() as *const c_char);

        println!("open_port_result:{}", open_port_result);

        let text_buffer = "hello world 阿三大苏打!\0".to_string();
        let gbk_data = GBK
            .encode(text_buffer.as_str(), EncoderTrap::Strict)
            .unwrap();
        let print_text_result = PrintText(
            printer_result,
            gbk_data.as_ptr() as *const c_char,
            2,
            text_buffer.len() as c_uint,
        );

        println!("print_text_result:{}", print_text_result);

        let tspl_feed_result = FeedLine(printer_result, 5);

        println!("tspl_feed_result:{}", tspl_feed_result);

        let release_printer_result = ReleasePrinter(printer_result);
        println!("ReleasePrinter:{:?}", release_printer_result);
    }
    Ok(())
}
