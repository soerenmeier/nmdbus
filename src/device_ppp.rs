// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DevicePpp {
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DevicePpp for blocking::Proxy<'a, C> {
}
