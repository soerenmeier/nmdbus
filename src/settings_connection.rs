// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait SettingsConnection {
    fn update(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error>;
    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error>;
    fn delete(&self) -> Result<(), dbus::Error>;
    fn get_settings(&self) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error>;
    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error>;
    fn clear_secrets(&self) -> Result<(), dbus::Error>;
    fn save(&self) -> Result<(), dbus::Error>;
    fn update2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<arg::PropMap, dbus::Error>;
    fn unsaved(&self) -> Result<bool, dbus::Error>;
}

#[derive(Debug)]
pub struct SettingsConnectionUpdated {
}

impl arg::AppendAll for SettingsConnectionUpdated {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for SettingsConnectionUpdated {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsConnectionUpdated {
        })
    }
}

impl dbus::message::SignalArgs for SettingsConnectionUpdated {
    const NAME: &'static str = "Updated";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}

#[derive(Debug)]
pub struct SettingsConnectionRemoved {
}

impl arg::AppendAll for SettingsConnectionRemoved {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for SettingsConnectionRemoved {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsConnectionRemoved {
        })
    }
}

impl dbus::message::SignalArgs for SettingsConnectionRemoved {
    const NAME: &'static str = "Removed";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}

#[derive(Debug)]
pub struct SettingsConnectionPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for SettingsConnectionPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for SettingsConnectionPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SettingsConnectionPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for SettingsConnectionPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> SettingsConnection for blocking::Proxy<'a, C> {

    fn update(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Update", (properties, ))
    }

    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "UpdateUnsaved", (properties, ))
    }

    fn delete(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Delete", ())
    }

    fn get_settings(&self) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "GetSettings", ())
            .and_then(|r: (::std::collections::HashMap<String, arg::PropMap>, )| Ok(r.0, ))
    }

    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "GetSecrets", (setting_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::PropMap>, )| Ok(r.0, ))
    }

    fn clear_secrets(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "ClearSecrets", ())
    }

    fn save(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Save", ())
    }

    fn update2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Update2", (settings, flags, args, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn unsaved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings.Connection", "Unsaved")
    }
}