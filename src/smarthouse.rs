use std::collections::HashMap;
use crate::smartdevices::SmartDevice;


pub struct SmartHouse {
    rooms: Vec<String>,
    devices: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            devices: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: &String) -> Result<(), String> {
        if self.rooms.contains(room) {
            return Err(format!("Комната с таким названием '{}' уже есть в доме", room));
        }
        self.rooms.push(room.clone());
        self.devices.insert(room.clone(), Vec::new());
        Ok(())
    }

    pub fn remove_room(&mut self, room: &String) -> Result<(), String> {
        if !self.rooms.contains(room) {
            return Err(format!("Комнаты с таким названием '{}' нет в доме", room));
        }
        self.rooms.retain(|x| x != room);
        self.devices.remove_entry(room).unwrap();
        Ok(())
    }

    pub fn get_rooms(&self) -> Option<&Vec<String>> {
        if self.rooms.is_empty() {
            None
        } else {
            Some(&self.rooms)
        }
    }

    pub fn devices(&self, room: &String) -> Result<Vec<String>, String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список устройств в комнате `room`")
        match room.as_str() {
            "кухня" => Ok(vec!["Розетка для чайника".to_string(), "?".to_string()]),
            "спальня" => Ok(vec![
                "Розетка для светильника".to_string(),
                "Термометр детский".to_string(),
            ]),
            _ => Err(format!("Неизвестное название комнаты: {}", room)),
        }
    }

    pub fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        device_info_provider: &impl DeviceInfoProvider,
    ) -> String {
        // todo!("перебор комнат и устройств в них для составления отчёта")
        let mut report = "".to_string();
        match self.get_rooms() {
            Some(rooms) => {
                for room in rooms.iter() {
                    match self.devices(room) {
                        Ok(devices) => {
                            for device in devices.iter() {
                                match device_info_provider.get_device_state(room, device) {
                                    Ok(device_status_report) => report.push_str(&device_status_report),
                                    Err(error) => report.push_str(&error),
                                }
                                report.push('\n') // Перевод строки
                            }
                        }
                        Err(error) => {
                            report.push_str(&format!("Ошибка получения списка устройств: {}", error));
                        }
                    }
                }
            }
            None => {
                report.push_str("В доме нет комнат");
            }
        }
        report
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствовать.
pub struct OwningDeviceInfoProvider {
    pub socket: crate::smartdevices::SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a crate::smartdevices::SmartSocket,
    pub thermo: &'b crate::smartdevices::SmartThermometer,
}

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room: &str, device: &str) -> Result<String, String>;
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> Result<String, String> {
        if device == self.socket.get_name() {
            Ok(format!(
                "Комната: '{}'; устройство: '{}'; {}",
                room,
                self.socket.get_name(),
                self.socket.get_state_info()
            ))
        } else {
            Err(format!(
                "Ошибка: неизвестное устройство '{}' в комнате '{}'",
                device, room
            ))
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_state(&self, room: &str, device: &str) -> Result<String, String> {
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
        }
        if report.is_empty() {
            Err(format!(
                "Ошибка: неизвестное устройство '{}' в комнате '{}'",
                device, room
            ))
        } else {
            Ok(report)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general_test() {
        let mut house = SmartHouse::new();
        house.add_room(&"кухня".to_string()).unwrap();
        house.add_room(&"спальня".to_string()).unwrap();
        assert_eq!(house.get_rooms().unwrap().len(), 2);
        assert_eq!(house.get_rooms().unwrap().first().unwrap(), "кухня");
        assert_eq!(house.devices(&"not-exist-room".to_string()).is_err(), true);

        house.add_device(&"кухня".to_string(),   vec!["Розетка для чайника".to_string(), "?".to_string()]);
        house.add_device(&"спальня".to_string(), vec![
                "Розетка для светильника".to_string(),
                "Термометр детский".to_string(),
            ]);
    }

    #[test]
    fn add_remove_room() {
        let mut house = SmartHouse::new();
        let test_room = &"test-room".to_string();
        assert_eq!(house.add_room(test_room).is_err(), false);
        assert_eq!(house.get_rooms().unwrap().len(), 1);
        assert_eq!(house.remove_room(&"not-exist-room".to_string()).is_err(), true);
        assert_eq!(house.remove_room(test_room).is_err(), false);
    }
}
