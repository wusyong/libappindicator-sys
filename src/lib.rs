#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate gdk_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk_sys;

use gdk_sys::GdkScrollDirection;
use glib_sys::{gboolean, gpointer, GType};
use gobject_sys::{GObject, GObjectClass};
use gtk_sys::{GtkMenu, GtkStatusIcon, GtkWidget};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
