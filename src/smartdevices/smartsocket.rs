use crate::smartdevices::smartdevice::SmartDevice;

pub struct SmartSocket {
    name: String,
    on: bool,
}

impl SmartSocket {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            on: false,
        }
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn is_on(&self) -> bool {
        self.on
    }

    fn set_on(&mut self, on: bool) {
        self.on = on
    }

    fn get_state_info(&self) -> String {
        if self.on {
            "состояние: включен".to_string()
        } else {
            "состояние: выключен".to_string()
        }
    }
}
