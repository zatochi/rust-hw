mod smartdevices;
use smartdevices::*;
mod smarthouse;
use smarthouse::*;

pub fn simple_example() {
    // Инициализация устройств
    let mut socket1 = SmartSocket::new("Розетка для чайника");
    let socket2 = SmartSocket::new("Розетка для светильника");
    let mut thermo = SmartThermometer::new("Термометр детский");
    socket1.set_on(true);
    thermo.set_temperature(36.6f32);

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, раскомментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, раскомментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smarthouse_smoketest() {
        simple_example();
    }
}
