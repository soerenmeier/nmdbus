// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait DeviceOvsInterface {
}

#[derive(Debug)]
pub struct DeviceOvsInterfacePropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for DeviceOvsInterfacePropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for DeviceOvsInterfacePropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceOvsInterfacePropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DeviceOvsInterfacePropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.OvsInterface";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> DeviceOvsInterface for blocking::Proxy<'a, C> {
}