/// Macro which ensures a certain closure will only be run once.
///
/// Usually used to avoid logging the same warning multiple times.
macro_rules! run_once {
    ($closure: expr) => {{
        use std::sync::Once;
        static LOGGED: Once = Once::new();
        LOGGED.call_once($closure);
    }};
}

macro_rules! impl_iunknown {
    (struct $struct_name:ty : $($ifaces:ident),*) => {
        #[implementation(IUnknown)]
        impl $struct_name {
            fn query_interface(&mut self, riid: &winapi::shared::guiddef::GUID, obj: &mut usize) -> i32 {
                use winapi::Interface;
                use winapi::shared::{guiddef::IsEqualGUID, winerror::{S_OK, E_NOTIMPL}};

                *obj = 0;

                if $(IsEqualGUID(riid, &$ifaces::uuidof())) || * {
                    *obj = self as *mut _ as usize;
                    self.add_ref();
                    S_OK
                } else {
                    E_NOTIMPL
                }
            }

            fn add_ref(&mut self) -> u32 {
                use std::sync::atomic::Ordering;
                let prev = self.refs.fetch_add(1, Ordering::SeqCst);
                prev + 1
            }

            fn release(&mut self) -> u32 {
                use std::sync::atomic::Ordering;
                let prev = self.refs.fetch_sub(1, Ordering::SeqCst);
                if prev == 1 {
                    let _box = unsafe { Box::from_raw(self as *mut _) };
                }
                prev - 1
            }
        }
    };
}

/// Helper Macro to "return err" or evaluate to Ok().
/// This is required to fail-fast with a HRESULT, since the ABI requires Error
/// and doesn't know Rust's Result types.
macro_rules! if_error {
    ($ex: expr) => {{
        match $ex {
            Err(err) => return err,
            Ok(res) => res,
        }
    }};
}

/// Helper Macro to "return err", if and only if the expression is not Error::Success
macro_rules! if_not_success {
    ($ex: expr) => {{
        match $ex {
            Error::Success => (),
            err => return err,
        }
    }};
}

/// Helper Macro to "return Err(err)", if and only if the expression is not Error::Success
macro_rules! if_not_success_err {
    ($ex: expr) => {{
        match $ex {
            Error::Success => (),
            err => return Err(err),
        }
    }};
}

/// Helper Macro, which evaluates to Error on Err and Error::Success on Ok().
macro_rules! to_error_success {
    ($ex: expr) => {{
        match $ex {
            Ok(_) => Error::Success,
            Err(e) => e,
        }
    }};
}
