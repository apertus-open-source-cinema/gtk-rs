// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Error;
use TlsDatabase;

glib_wrapper! {
    pub struct TlsFileDatabase(Interface<gio_sys::GTlsFileDatabase>) @requires TlsDatabase;

    match fn {
        get_type => || gio_sys::g_tls_file_database_get_type(),
    }
}

impl TlsFileDatabase {
    pub fn new<P: AsRef<std::path::Path>>(anchors: P) -> Result<TlsFileDatabase, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_tls_file_database_new(anchors.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_TLS_FILE_DATABASE: Option<&TlsFileDatabase> = None;

pub trait TlsFileDatabaseExt: 'static {
    fn get_property_anchors(&self) -> Option<GString>;

    fn set_property_anchors(&self, anchors: Option<&str>);

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsFileDatabase>> TlsFileDatabaseExt for O {
    fn get_property_anchors(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"anchors\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `anchors` getter")
        }
    }

    fn set_property_anchors(&self, anchors: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"anchors\0".as_ptr() as *const _,
                Value::from(anchors).to_glib_none().0,
            );
        }
    }

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchors_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GTlsFileDatabase,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TlsFileDatabase>,
        {
            let f: &F = &*(f as *const F);
            f(&TlsFileDatabase::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anchors\0".as_ptr() as *const _,
                Some(transmute(notify_anchors_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsFileDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsFileDatabase")
    }
}
