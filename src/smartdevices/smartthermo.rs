use crate::smartdevices::smartdevice::SmartDevice;

pub struct SmartThermometer {
    name: String,
    on: bool,
    temperature: f32,
}

impl SmartThermometer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            on: false,
            temperature: 0f32,
        }
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl SmartDevice for SmartThermometer {
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
            format!("состояние: включен; температура: {} градусов", self.temperature)
        } else {
            "состояние: выключен".to_string()
        }
    }
}
