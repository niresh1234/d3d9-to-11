use crate::core::Context;
use crate::Error;
use comptr::ComPtr;
use std::sync::Once;
use widestring::U16CStr;
use winapi::shared::d3d9::IDirect3D9Ex;
use winapi::shared::d3d9types::D3DCOLOR;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LPCWSTR;

static INIT_LOGGER: Once = Once::new();
const DX9C_DEBUG: u32 = 0x80000000 | 32;

// Note: the original code had extern "system", which is "stdcall" on Win32, but the original code
// had some special mingw linker flags to "fix up stdcall", maybe because "system" is cdecl there.
// Either way, WinAPI is supposed to be stdcall, regardless of any toolchain

/// # Safety
/// This is unsafe because it's exported as DLL
#[no_mangle]
pub unsafe extern "stdcall" fn Direct3DCreate9(sdk_version: u32) -> Option<ComPtr<Context>> {
    // This function could be called multiple times during the lifetime of the DLL,
    // so we must protect the logger initializer.
    INIT_LOGGER.call_once(|| {
        #[cfg(feature = "env_logger")]
        env_logger::init();
        #[cfg(feature = "win_dbg_logger")]
        win_dbg_logger::init();
    });

    // Try to identify which version of the D3D9 the app was built against.
    // This could be used to implement compatibility workarounds if needed.
    // If the bit 0x80000000 is set, it's debug
    run_once!(|| match sdk_version {
        32 => info!("D3D9 version 9.0c"),
        DX9C_DEBUG => info!("D3D9 version 9.0c Debug Mode"),
        _ => warn!("Unknown D3D9 SDK version {}", sdk_version),
    });

    Context::new().ok()
}

/// # Safety
/// This is unsafe because it's exported as DLL
#[no_mangle]
pub unsafe extern "stdcall" fn Direct3DCreate9Ex(
    _sdk_version: u32,
    _ptr: *mut *mut IDirect3D9Ex,
) -> Error {
    error!("D3D9Ex is not yet supported");
    Error::NotAvailable
}

/// # Safety
/// This is unsafe because it's exported as DLL
#[no_mangle]
pub unsafe extern "stdcall" fn D3DPERF_BeginEvent(col: D3DCOLOR, wsz_name: LPCWSTR) -> i32 {
    let name = U16CStr::from_ptr_str(wsz_name);
    warn!("BeginEvent({}, {})", col, name.to_string_lossy());
    0
}

/// # Safety
/// This is unsafe because it's exported as DLL
#[no_mangle]
pub unsafe extern "stdcall" fn D3DPERF_SetOptions(dw_options: DWORD) -> i32 {
    // if dw_options = 1, no permission to be profiled
    0 // This function doesn't return a value
}

/// # Safety
/// This is unsafe because it's exported as DLL
#[no_mangle]
pub unsafe extern "stdcall" fn D3DPERF_EndEvent() -> i32 {
    0
}
