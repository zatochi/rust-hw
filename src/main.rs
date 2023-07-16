#[derive(Debug, Clone)]
struct Device {
    name: String,
}

struct Room {
    name: String,
    devices: Vec<Device>,
}

struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|room| room.name.clone()).collect()
    }
    
    fn get_devices_in_room(&self, room_name: &str) -> Option<Vec<String>> {
        self.rooms
            .iter()
            .find(|room| room.name == room_name)
            .map(|room| room.devices.iter().map(|device| device.name.clone()).collect())
    }
    
    fn get_device_state<T>(&self, room_name: &str, device_name: &str) -> Result<T, String>
    where
        T: ToString,
    {
        self.rooms
            .iter()
            .find(|room| room.name == room_name)
            .and_then(|room| {
                room.devices
                    .iter()
                    .find(|device| device.name == device_name)
                    .map(|device| device.name.to_string())
            })
            .ok_or_else(|| format!("Device '{}' not found in room '{}'", device_name, room_name))
            .map(|device_name| T::to_string(&device_name) as T)
    }
}

fn main() {
    let mut devices = vec![
        Device { name: "Light".to_string() },
        Device { name: "Fan".to_string() },
    ];
    
    let mut rooms = vec![
        Room { name: "Living Room".to_string(), devices: devices.clone() },
        Room { name: "Bedroom".to_string(), devices: devices.clone() },
    ];
    
    let house = House { name: "My House".to_string(), rooms };
    
    let rooms = house.get_rooms();
    println!("Rooms: {:?}", rooms);

    let devices = house.get_devices_in_room("Living Room");
    println!("Devices in Living Room: {:?}", devices);

    let device_state: Result<String, _> = house.get_device_state("Living Room", "Light");

    match device_state {
        Ok(state) => println!("Light state in Living Room: {}", state),
        Err(error) => println!("Error: {}", error),
    }
}
