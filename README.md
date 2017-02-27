# libappindicator-sys

Bindings for the libappindicator library. libappindicator provides
cross-distribution/window system functions for creating systray icons
and menus.

Note that rather than generate bindings for all gtk types,
libappindicator-sys uses some of the types out of gtk-sys for things
like function argument types. This allows the bindings to be used with
the higher level gtk-rs crate.
