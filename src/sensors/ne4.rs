use std::fmt;
use tokio::time::*;
use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};

/// Representation des 'NE4-MOD-BUS' Sensors
///
/// Quelle:
/// [RA-GAS-Modbus-Systembeschreibung: Stand 09.04.2020 (Firmware 9040)](09-04-2020_Beschreibung_RA-GAS_Sensor-MB.docx)
#[derive(Debug)]
pub struct NE4 {
    rreg: Vec<u16>,
}

impl NE4 {
    pub fn new() -> Self {
        NE4 {
            rreg: vec![0u16; 50],
        }
    }

    pub async fn update(&mut self) -> Result<(), std::io::Error> {
        for (i, reg) in self.rreg.iter_mut().enumerate() {
            let value = timeout(Duration::from_millis(100), read_register(i)).await?;
            *reg = value.unwrap()[0];
        }
        Ok(())
    }

    /// Rreg
    /// Gerätekennung Kunden
    pub fn device_id(&self) -> u16 {
        self.rreg[0]
    }

    /// Arbeitsweise (Sensor)
    pub fn work_mode(&self) -> u16 {
        self.rreg[1]
    }

    /// Gaskonzentration in ppm
    pub fn concentration_gas(&self) -> u16 {
        self.rreg[2]
    }

    /// Berechneter Ausgangsstrom in mA (mit zwei Kommastellen)
    pub fn output_current(&self) -> u16 {
        self.rreg[3]
    }

    /// Interne Leiterplattentemperatur in °C (mit Kommastelle)
    pub fn int_temp(&self) -> u16 {
        self.rreg[4]
    }

    /// AD-Wert der Temperaturmessung
    pub fn adc_temp(&self) -> u16 {
        self.rreg[40]
    }

    /// AD-Wert des Potentiometers
    pub fn adc_poti(&self) -> u16 {
        self.rreg[41]
    }

    /// AD-Wert des Sensors
    pub fn adc_sensor(&self) -> u16 {
        self.rreg[42]
    }

    /// Verstärkungsfaktor durch Poti (100 = 1,00)
    pub fn amplification_poti(&self) -> u16 {
        self.rreg[43]
    }

    /// Verstärkungsfaktor durch Temperaturkennlinie (100 = 1,00)
    pub fn amplification_temp(&self) -> u16 {
        self.rreg[44]
    }

    /// Korrigierter AD-Wert des Sensors
    pub fn adc_sensor_corrected(&self) -> u16 {
        self.rreg[45]
    }

    /// berechnete Gaskonzentration im ppm
    pub fn concentration_gas_calculated(&self) -> u16 {
        self.rreg[46]
    }

    /// Softwaredatum bis 31.12.2029
    pub fn software_date(&self) -> u16 {
        self.rreg[49]
    }
}

impl fmt::Display for NE4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NE4-MOD-BUS")
    }
}

async fn read_register(i: usize) -> Result<Vec<u16>, futures::io::Error> {
    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(247);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();

    let mut ctx = rtu::connect_slave(port, slave).await?;
    ctx.read_holding_registers(i as u16, 1).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiring() {
        let ne4 = NE4::new();
        assert_eq!(ne4.rreg.len(), 50);
    }

    // #[tokio::test]
    // async fn update() {
    //     let mut ne4 = NE4::new();
    //     assert_eq!(*ne4.rreg.first().unwrap(), 0u16);
    //     assert_eq!(*ne4.rreg.last().unwrap(), 0u16);
    //     ne4.update().await.unwrap();
    //     assert_eq!(*ne4.rreg.first().unwrap(), 0);
    //     assert_eq!(*ne4.rreg.last().unwrap(), 0);
    // }

    /// Test Rreg functions
    #[test]
    fn device_id() {
        let mut ne4 = NE4::new();
        ne4.rreg[0] = 0x1000;
        assert_eq!(ne4.device_id(), 4096);
    }

    #[test]
    fn work_mode() {
        let mut ne4 = NE4::new();
        ne4.rreg[1] = 0x1000;
        assert_eq!(ne4.work_mode(), 4096);
    }
    #[test]
    fn concentration_gas() {
        let mut ne4 = NE4::new();
        ne4.rreg[2] = 0x1000;
        assert_eq!(ne4.concentration_gas(), 4096);
    }
    #[test]
    fn output_current() {
        let mut ne4 = NE4::new();
        ne4.rreg[3] = 0x1000;
        assert_eq!(ne4.output_current(), 4096);
    }
    #[test]
    fn int_temp() {
        let mut ne4 = NE4::new();
        ne4.rreg[4] = 0x1000;
        assert_eq!(ne4.int_temp(), 4096);
    }
    #[test]
    fn adc_temp() {
        let mut ne4 = NE4::new();
        ne4.rreg[40] = 0x1000;
        assert_eq!(ne4.adc_temp(), 4096);
    }
    #[test]
    fn adc_poti() {
        let mut ne4 = NE4::new();
        ne4.rreg[41] = 0x1000;
        assert_eq!(ne4.adc_poti(), 4096);
    }
    #[test]
    fn adc_sensor() {
        let mut ne4 = NE4::new();
        ne4.rreg[42] = 0x1000;
        assert_eq!(ne4.adc_sensor(), 4096);
    }
    #[test]
    fn amplification_poti() {
        let mut ne4 = NE4::new();
        ne4.rreg[43] = 0x1000;
        assert_eq!(ne4.amplification_poti(), 4096);
    }
    #[test]
    fn amplification_temp() {
        let mut ne4 = NE4::new();
        ne4.rreg[44] = 0x1000;
        assert_eq!(ne4.amplification_temp(), 4096);
    }
    #[test]
    fn adc_sensor_corrected() {
        let mut ne4 = NE4::new();
        ne4.rreg[45] = 0x1000;
        assert_eq!(ne4.adc_sensor_corrected(), 4096);
    }
    #[test]
    fn concentration_gas_calculated() {
        let mut ne4 = NE4::new();
        ne4.rreg[46] = 0x1000;
        assert_eq!(ne4.concentration_gas_calculated(), 4096);
    }
    #[test]
    fn software_date() {
        let mut ne4 = NE4::new();
        ne4.rreg[49] = 0x1000;
        assert_eq!(ne4.software_date(), 4096);
    }
}
