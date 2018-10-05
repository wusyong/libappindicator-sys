#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate gtk_sys;

use gtk_sys::{
    GtkContainer, GtkContainerPrivate, GtkMenu, GtkMenuPrivate, GtkMenuShell, GtkMenuShellPrivate,
    GtkStatusIcon, GtkStatusIconPrivate, GtkWidget, GtkWidgetPrivate,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
