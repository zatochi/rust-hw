mod smartdevice;
pub use smartdevice::*;

// Пользовательские устройства:
mod smartsocket;
pub use smartsocket::*;
mod smartthermo;
pub use smartthermo::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_smoketest() {
        let mut socket = SmartSocket::new("socket");
        assert_eq!(socket.get_name(), "socket");
        assert_eq!(socket.is_on(), false);
        socket.set_on(true);
        assert_eq!(socket.is_on(), true);
    }

    #[test]
    fn thermo_smoketest() {
        let mut thermo = SmartThermometer::new("thermo");
        assert_eq!(thermo.get_name(), "thermo");
        assert_eq!(thermo.is_on(), false);
        assert_eq!(thermo.get_temperature(), 0.0f32);

        let temperature = 1.0f32 / 3.0f32;
        thermo.set_temperature(temperature);
        assert!((temperature - thermo.get_temperature()).abs() < 1e-6f32);
    }
}
