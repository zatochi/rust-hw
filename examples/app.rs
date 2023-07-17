use prelude::*;

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