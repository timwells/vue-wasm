extern crate qrcode;
use qrcode::render::svg;
use qrcode::QrCode;
use qrcode::types::QrError;

/// Generate a QR code from the respective data. Returns a string containing the SVG string
fn qrcode<T>(data: T, width: u32, height: u32) -> Result<String, QrError>
where T: AsRef<[u8]> {
    QrCode::with_error_correction_level(data.as_ref(), qrcode::EcLevel::Q)
        .map(|code| code.render::<svg::Color>()
            .max_dimensions(width, height)
            .min_dimensions(width, height)
            .build()
        )
}

/// Returns a new pointer to a new location in memory where the SVG code for the qrcode 
/// as a base64 is located.
pub fn qrcode_ffi(arg: &str, width: u32, height: u32) -> String {
    match qrcode(arg, width, height) {
        Ok(v) => format!("{}{}","data:image/svg+xml;base64,", base64::encode(v)),
        // Since we're on an FFI boundary we can't return strongly typed errors. Instead if we get
        // an error from the qrcode generation we return the error string.
        Err(e) => format!("{}", e),
    }
}