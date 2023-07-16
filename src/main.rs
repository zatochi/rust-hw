// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room: &str, device: &str) -> String;
}

struct SmartHouse {
    /* todo: данные умного дома */
    rooms: [String; 2],
}

impl SmartHouse {
    fn new() -> Self {
        // todo!("реализовать инициализацию дома")
        Self {
            rooms: ["кухня".to_string(), "спальня".to_string()],
        }
    }

    fn get_rooms(&self) -> [String; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список комнат");
        self.rooms.clone()
    }

    fn devices(&self, room: &str) -> [String; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список устройств в комнате `room`")
        match room {
            "кухня" => ["Розетка для чайника".to_string(), "?".to_string()],
            "спальня" => [
                "Розетка для светильника".to_string(),
                "Термометр детский".to_string(),
            ],
            _ => panic!("Неизвестное название комнаты: {}", room),
        }
    }

    fn create_report(
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

trait SmartDeviceStateInfo {
    fn get_name(&self) -> &String;
    fn get_state_info(&self) -> String;
}

// Пользовательские устройства:
struct SmartSocket {
    name: String,
    on: bool,
}
struct SmartThermometer {
    name: String,
    on: bool,
    temperature: f32,
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

impl SmartDeviceStateInfo for SmartThermometer {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_state_info(&self) -> String {
        if self.on {
            format!(
                "состояние: включен; температура: {} градусов",
                self.temperature
            )
        } else {
            "состояние: выключен".to_string()
        }
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
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
            format!(
                "Ошибка: неизвестное устройство '{}' в комнате '{}'",
                device, room,
            )
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

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: "Розетка для чайника".to_string(),
        on: true,
    };
    let socket2 = SmartSocket {
        name: "Розетка для светильника".to_string(),
        on: false,
    };
    let thermo = SmartThermometer {
        name: "Термометр детский".to_string(),
        on: true,
        temperature: 36.6,
    };

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");
}
