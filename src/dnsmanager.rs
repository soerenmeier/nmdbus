// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DnsManager {
    fn mode(&self) -> Result<String, dbus::Error>;
    fn rc_manager(&self) -> Result<String, dbus::Error>;
    fn configuration(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DnsManager for blocking::Proxy<'a, C> {

    fn mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.DnsManager", "Mode")
    }

    fn rc_manager(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.DnsManager", "RcManager")
    }

    fn configuration(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.DnsManager", "Configuration")
    }
}