pub mod smart_house {
    pub trait DeviceInfoProvider {
        // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
        fn get_device_state(&self, room: &str, device: &str) -> String;
    }

    pub struct SmartHouse {
        /* todo: данные умного дома */
        rooms: [String; 2],
    }

    impl SmartHouse {
        pub fn new() -> Self {
            // todo!("реализовать инициализацию дома")
            Self {
                rooms: ["кухня".to_string(), "спальня".to_string()],
            }
        }

        pub fn get_rooms(&self) -> [String; 2] {
            // Размер возвращаемого массива можно выбрать самостоятельно
            // todo!("список комнат");
            self.rooms.clone()
        }

        pub fn devices(&self, room: &str) -> [String; 2] {
            // Размер возвращаемого массива можно выбрать самостоятельно
            // todo!("список устройств в комнате `room`")
            match room {
                "кухня" => ["Розетка для чайника".to_string(), "?".to_string()],
                "спальня" => ["Розетка для светильника".to_string(), "Термометр детский".to_string()],
                _ => panic!("Неизвестное название комнаты: {}", room),
            }
        }

        pub fn create_report(
            &self,
            /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
            device_info_provider: &impl DeviceInfoProvider,
        ) -> String {
            // todo!("перебор комнат и устройств в них для составления отчёта")
            let mut report = "".to_string();
            for room in self.get_rooms().iter() {
                for device in self.devices(room).iter() {
                    report.push_str(&device_info_provider.get_device_state(room, device));
                    report.push('\n') // Перевод строки
                }
            }
            report
        }
    }

    // ***** Пример использования библиотеки умный дом:

    pub trait SmartDeviceStateInfo {
        fn get_name(&self) -> &String;
        fn get_state_info(&self) -> String;
    }

    // Пользовательские устройства:
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

        pub fn set_on(&mut self, on: bool) {
            self.on = on
        }

        pub fn is_on(&self) -> bool {
            self.on
        }
    }

    impl SmartDeviceStateInfo for SmartSocket {
        fn get_name(&self) -> &String {
            &self.name
        }

        fn get_state_info(&self) -> String {
            if self.on {
                "состояние: включен".to_string()
            } else {
                "состояние: выключен".to_string()
            }
        }
    }

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
                temperature: 0 as f32,
            }
        }

        pub fn set_on(&mut self, on: bool) {
            self.on = on
        }

        pub fn is_on(&self) -> bool {
            self.on
        }
    }

    impl SmartDeviceStateInfo for SmartThermometer {
        fn get_name(&self) -> &String {
            &self.name
        }

        fn get_state_info(&self) -> String {
            if self.on {
                format!("состояние: включен; температура: {} градусов", self.temperature)
            } else {
                "состояние: выключен".to_string()
            }
        }
    }

    // Пользовательские поставщики информации об устройствах.
    // Могут как хранить устройства, так и заимствывать.
    pub struct OwningDeviceInfoProvider {
        socket: SmartSocket,
    }

    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        socket: &'a SmartSocket,
        thermo: &'b SmartThermometer,
    }

    // todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn get_device_state(&self, room: &str, device: &str) -> String {
            if device == self.socket.get_name() {
                format!(
                    "Комната: '{}'; устройство: '{}'; {}",
                    room,
                    self.socket.get_name(),
                    self.socket.get_state_info()
                )
            } else {
                format!("Ошибка: неизвестное устройство '{}' в комнате '{}'", device, room,)
            }
        }
    }

    impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
        fn get_device_state(&self, room: &str, device: &str) -> String {
            let mut report = "".to_string();
            if device == self.socket.get_name() {
                report.push_str(&format!(
                    "Комната: '{}'; устройство: '{}'; {}",
                    room,
                    self.socket.get_name(),
                    self.socket.get_state_info()
                ));
            } else if device == self.thermo.get_name() {
                report.push_str(&format!(
                    "Комната: '{}'; устройство: '{}'; {}",
                    room,
                    self.thermo.get_name(),
                    self.thermo.get_state_info()
                ));
            } else {
                report.push_str(&format!(
                    "Ошибка: неизвестное устройство '{}' в комнате '{}'",
                    device, room,
                ));
            }
            report
        }
    }
}

#[cfg(test)]
mod test {
    use crate::smart_house::*;

    #[test]
    fn general_test() {
        let socket = SmartSocket::new("socket");
        let mut thermo = SmartThermometer::new("thermo");
        let house = SmartHouse::new();

        assert_eq!(socket.get_name(), "socket");
        assert_eq!(socket.is_on(), false);

        assert_eq!(thermo.get_name(), "thermo");
        assert_eq!(thermo.is_on(), false);
        thermo.set_on(true);
        assert_eq!(thermo.is_on(), true);

        assert_eq!(house.get_rooms().len(), 2);
    }
}