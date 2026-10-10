//! Physical pixels per inch of the primary display, for Preferences › Display Print Size at 100% Zoom.
//!
//! View → Actual Size uses this when the preference is on so one document inch fills one physical
//! inch on the screen. When the OS does not report a size, the UI keeps the CSS reference density
//! (96).

/// Physical pixels per inch of the primary monitor, when the platform can measure it.
pub fn primary() -> Option<f64> {
    #[cfg(target_os = "macos")]
    {
        macos_primary()
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

#[cfg(target_os = "macos")]
fn macos_primary() -> Option<f64> {
    #[repr(C)]
    struct CGSize {
        width: f64,
        #[allow(dead_code)]
        height: f64,
    }
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGMainDisplayID() -> u32;
        fn CGDisplayPixelsWide(display: u32) -> usize;
        fn CGDisplayScreenSize(display: u32) -> CGSize;
    }
    // SAFETY: CoreGraphics display query APIs; the main display id is always valid to pass.
    let (px, mm) = unsafe {
        let id = CGMainDisplayID();
        (CGDisplayPixelsWide(id) as f64, CGDisplayScreenSize(id).width)
    };
    if px.is_finite() && mm.is_finite() && px > 0.0 && mm > 0.0 {
        Some(px / (mm / 25.4))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primary_is_finite_when_reported() {
        if let Some(ppi) = super::primary() {
            assert!(ppi.is_finite() && ppi > 10.0 && ppi < 1000.0, "{ppi}");
        }
    }
}
