// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DeviceOvsBridge {
}

#[derive(Debug)]
pub struct DeviceOvsBridgePropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for DeviceOvsBridgePropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for DeviceOvsBridgePropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceOvsBridgePropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceOvsBridgePropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.OvsBridge";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DeviceOvsBridge for blocking::Proxy<'a, C> {
}