#![cfg(feature = "NSValue")]
use alloc::format;
use core::ffi::c_char;
use core::ffi::CStr;
use core::slice;
use core::str;

use crate::NSValue;

#[test]
fn basic() {
    let val = NSValue::new(13u32);
    assert_eq!(unsafe { val.get::<u32>() }, 13);
}

#[test]
fn test_equality() {
    let val1 = NSValue::new(123u32);
    let val2 = NSValue::new(123u32);
    assert_eq!(val1, val1);
    assert_eq!(val1, val2);

    let val3 = NSValue::new(456u32);
    assert_ne!(val1, val3);
}

#[test]
fn test_equality_across_types() {
    let val1 = NSValue::new(123i32);
    let val2 = NSValue::new(123u32);

    // Test that `objCType` is checked when comparing equality
    assert_ne!(val1, val2);
}

#[test]
fn test_debug() {
    // Output changes depending on OS and OS version
    let expected = [
        r#"NSValue { encoding: "C", bytes: (C) <ab> }"#,
        r#"NSValue { encoding: "C", bytes: {length = 1, bytes = 0xab} }"#,
        r#"NSValue { encoding: "C", bytes: <ab> }"#,
    ];
    let actual = format!("{:?}", NSValue::new(171u8));
    assert!(expected.contains(&&*actual), "{actual}");
}

#[test]
#[cfg(feature = "NSRange")]
fn nsrange() {
    use crate::NSRange;
    let range = NSRange::from(1..2);
    let val = NSValue::new(range);
    assert_eq!(val.get_range(), Some(range));
    #[cfg(feature = "objc2-core-foundation")]
    {
        assert_eq!(val.get_point(), None);
        assert_eq!(val.get_size(), None);
        assert_eq!(val.get_rect(), None);
    }
    // NSValue -getValue is broken on GNUStep for some types
    #[cfg(not(feature = "gnustep-1-7"))]
    assert_eq!(unsafe { val.get::<NSRange>() }, range);
}

#[test]
#[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
fn nspoint() {
    use crate::NSPoint;
    let point = NSPoint::new(1.0, 2.0);
    let val = NSValue::new(point);
    assert_eq!(val.get_point(), Some(point));
    #[cfg(not(feature = "gnustep-1-7"))]
    assert_eq!(unsafe { val.get::<NSPoint>() }, point);
}

#[test]
#[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
fn nssize() {
    use crate::NSSize;
    let point = NSSize::new(1.0, 2.0);
    let val = NSValue::new(point);
    assert_eq!(val.get_size(), Some(point));
    #[cfg(not(feature = "gnustep-1-7"))]
    assert_eq!(unsafe { val.get::<NSSize>() }, point);
}

#[test]
#[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
fn nsrect() {
    use crate::{NSPoint, NSRect, NSSize};
    let rect = NSRect::new(NSPoint::new(1.0, 2.0), NSSize::new(3.0, 4.0));
    let val = NSValue::new(rect);
    assert_eq!(val.get_rect(), Some(rect));
    #[cfg(not(feature = "gnustep-1-7"))]
    assert_eq!(unsafe { val.get::<NSRect>() }, rect);
}

#[test]
fn store_str() {
    let s = "abc";
    let val = NSValue::new(s.as_ptr());
    assert!(val.contains_encoding::<*const u8>());
    let slice = unsafe { slice::from_raw_parts(val.get(), s.len()) };
    let s2 = str::from_utf8(slice).unwrap();
    assert_eq!(s2, s);
}

#[test]
fn store_cstr() {
    // The following Apple article says that NSValue can't easily store
    // C-strings, but apparently that doesn't apply to us!
    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/NumbersandValues/Articles/Values.html#//apple_ref/doc/uid/20000174-BAJJHDEG>
    let s = CStr::from_bytes_with_nul(b"test123\0").unwrap();
    let val = NSValue::new(s.as_ptr());
    assert!(val.contains_encoding::<*const c_char>());
    let s2 = unsafe { CStr::from_ptr(val.get()) };
    assert_eq!(s2, s);
}
