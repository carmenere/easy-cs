use crate::gui::{Form};
use crate::linux::{LinuxForm};
use crate::macos::{MacOSForm};

pub enum OS {
    Linux,
    MacOS
}

pub struct FormsFactory {}

impl FormsFactory {
    pub fn new(typ: OS) -> Box<dyn Form> {
        match typ {
            OS::Linux => {
                Box::new(LinuxForm {})
            },
            OS::MacOS => {
                Box::new(MacOSForm {})
            },
        }
    }
}