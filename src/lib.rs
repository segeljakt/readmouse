use std::os::raw::c_void;

type CFTypeRef = *const c_void;
type CGEventRef = *const c_void;
type CGEventSourceRef = *const c_void;

#[derive(Clone, Copy)]
#[repr(C)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    fn CFRelease(cf: CFTypeRef);
}

pub struct Mouse;

impl Mouse {
    /// Returns the current mouse location.
    /// ```
    /// use readmouse::Mouse;
    /// loop {
    ///   println!("State of Up key: {}, ", Mouse::location();
    /// }
    /// ```
    #[inline(always)]
    pub fn location() -> (f64, f64) {
        unsafe {
            let event = CGEventCreate(std::ptr::null());
            let CGPoint { x, y } = CGEventGetLocation(event);
            CFRelease(event);
            (x, y)
        }
    }
}
