pub trait SmartDevice {
    // Получить название устройства
    fn get_name(&self) -> &String;

    // Устройство включено?
    fn is_on(&self) -> bool;

    // Включить устройство
    fn set_on(&mut self, on: bool);

    // Получить сводку о состоянии устройства в текстовом виде
    fn get_state_info(&self) -> String;
}
