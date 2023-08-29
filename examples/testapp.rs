// ***** Пример использования библиотеки умный дом:
use std::io::{self, Write};
use smarthouse::*;


struct UserInput;

fn show_err_msg(msg: &str) {
    // Вывод сообщения красным цветом
    println!("\x1b[31m{}\x1b[0m", msg);
}

impl UserInput {
    fn show_prompt(prompt: &str) {
        println!("{}: ", prompt);
        print!("> ");
        io::stdout().flush().unwrap();
    }

    fn prompt_positive_number(prompt: &str) -> u32 {
        loop {
            Self::show_prompt(prompt);
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let result = buffer.trim().parse::<u32>();
            if result.is_err() {
                show_err_msg("Ожидается ввод положительного числа!");
            } else {
                return result.ok().unwrap()
            }
        }
    }

    fn prompt_not_empty_string(prompt: &str) -> String {
        loop {
            Self::show_prompt(prompt);
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let trimmed = buffer.trim();
            if trimmed.is_empty() {
                show_err_msg("Ожидается ввод не пустой строки!");
            }
            else {
                return String::from(trimmed);
            }
        }
    }
}


enum CliCommands {
    Quit = 0,
    ShowReport = 1,
    ShowRoomList = 2,
    AddRoom = 3,
    RemoveRoom = 4,
    ShowDeviceList = 5,
    AddDevice = 6,
    RemoveDevice = 7,
}

impl CliCommands {
    fn as_number(self) -> u32 {
        self as u32
    }

    fn is_same_number(self, val: u32) -> bool {
        self.as_number() == val
    }
}

// Консольный интерактивный пример
fn cli_example() {
    // Инициализация устройств
    //let mut socket1 = SmartSocket::new("Розетка для чайника");
    //let socket2 = SmartSocket::new("Розетка для светильника");
    //let mut thermo = SmartThermometer::new("Термометр детский");
    //socket1.set_on(true);
    //thermo.set_temperature(36.6f32);
    //let _ = thermo.get_temperature(); // Делаем линтер счастливым
    //// Инициализация дома
    //let house = SmartHouse::new();
    //let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    loop {
        println!("\n\n---------------------------------------");
        println!("{}", "Доступные команды:");
        println!("{}) Показать отчет", CliCommands::ShowReport.as_number());
        println!("{}) Показать список комнат", CliCommands::ShowRoomList.as_number());
        println!("{}) Добавить комнату", CliCommands::AddRoom.as_number());
        println!("{}) Удалить комнату", CliCommands::RemoveRoom.as_number());
        println!("{}) Показать список устройств в комнате", CliCommands::ShowDeviceList.as_number());
        println!("{}) Добавить устройство", CliCommands::AddDevice.as_number());
        println!("{}) Удалить устройство", CliCommands::RemoveDevice.as_number());
        println!("{}) Выйти", CliCommands::Quit.as_number());
        println!("");
        let num = UserInput::prompt_positive_number("Выберите номер команды");
        let mut valid_cli_command = false;

        if CliCommands::ShowReport.is_same_number(num) {
            valid_cli_command = true;
            //let report = house.create_report(&info_provider_1);
            //println!("{}", report);
        }

        if CliCommands::ShowRoomList.is_same_number(num) {
            valid_cli_command = true;
            print!("Отображение списка комнат..");
        }

        if CliCommands::AddRoom.is_same_number(num) {
            valid_cli_command = true;
            let room = UserInput::prompt_not_empty_string("Укажите название комнаты");
            print!("Добавление комнаты '{}'..", room);
        }

        if CliCommands::RemoveRoom.is_same_number(num) {
            valid_cli_command = true;
            let room = UserInput::prompt_not_empty_string("Укажите название комнаты");
            print!("Удаление комнаты '{}'..", room);
        }

        if CliCommands::ShowDeviceList.is_same_number(num) {
            valid_cli_command = true;
            let room = UserInput::prompt_not_empty_string("Укажите название комнаты");
            print!("Отображение списка устройств в комнате '{}'..", room);
        }

        if CliCommands::AddDevice.is_same_number(num) {
            valid_cli_command = true;
            let device = UserInput::prompt_not_empty_string("Укажите название устройства");
            let room = UserInput::prompt_not_empty_string("Укажите название комнаты");
            print!("Добавление устройства '{}' в комнату '{}'..", device, room);
        }

        if CliCommands::RemoveDevice.is_same_number(num) {
            valid_cli_command = true;
            let device = UserInput::prompt_not_empty_string("Укажите название устройства");
            let room = UserInput::prompt_not_empty_string("Укажите название комнаты");
            print!("Удаление устройства '{}' из комнаты '{}'..", room, device);
        }

        if CliCommands::Quit.is_same_number(num) {
            println!("Выход..");
            break
        }

        if !valid_cli_command {
            show_err_msg("Указан неверный номер команды!");
        }
    }
}


fn main() {
    cli_example();
}
