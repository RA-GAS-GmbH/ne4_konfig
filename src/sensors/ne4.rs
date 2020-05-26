
/// Representation des 'NE4-MOD-BUS' Sensors
///
/// Quelle:
/// [RA-GAS-Modbus-Systembeschreibung: Stand 09.04.2020 (Firmware 9040)](09-04-2020_Beschreibung_RA-GAS_Sensor-MB.docx)
pub struct NE4 {
    registers: Vec<u16>,
}

impl NE4 {
    pub fn new() -> Self {
        NE4 {
            registers: vec![0u16; 100],
        }
    }

    pub fn update(&mut self) {
        for (i, reg) in self.registers.iter_mut().enumerate() {
            *reg = i as u16;
        }
    }

    /// Rreg
    /// Gerätekennung Kunden
    pub fn device_id(&self) -> u16 {
        self.registers[0]
    }

    /// Arbeitsweise (Sensor)
    pub fn work_mode(&self) -> u16 {
        self.registers[1]
    }

    /// Gaskonzentration in ppm
    pub fn concentration_gas(&self) -> u16 {
        self.registers[2]
    }

    /// Berechneter Ausgangsstrom in mA (mit zwei Kommastellen)
    pub fn output_current(&self) -> u16 {
        self.registers[3]
    }

    /// Interne Leiterplattentemperatur in °C (mit Kommastelle)
    pub fn int_temp(&self) -> u16 {
        self.registers[4]
    }

    /// AD-Wert der Temperaturmessung
    pub fn adc_temp(&self) -> u16 {
        self.registers[40]
    }

    /// AD-Wert des Potentiometers
    pub fn adc_poti(&self) -> u16 {
        self.registers[41]
    }

    /// AD-Wert des Sensors
    pub fn adc_sensor(&self) -> u16 {
        self.registers[42]
    }

    /// Verstärkungsfaktor durch Poti (100 = 1,00)
    pub fn amplification_poti(&self) -> u16 {
        self.registers[43]
    }

    /// Verstärkungsfaktor durch Temperaturkennlinie (100 = 1,00)
    pub fn amplification_temp(&self) -> u16 {
        self.registers[44]
    }

    /// Korrigierter AD-Wert des Sensors
    pub fn adc_sensor_corrected(&self) -> u16 {
        self.registers[45]
    }

    /// berechnete Gaskonzentration im ppm
    pub fn concentration_gas_calculated(&self) -> u16 {
        self.registers[46]
    }

    /// Softwaredatum bis 31.12.2029
    pub fn software_date(&self) -> u16 {
        self.registers[49]
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiring() {
        let ne4 = NE4::new();
        assert_eq!(ne4.registers.len(), 100);
    }

    #[test]
    fn update() {
        let mut ne4 = NE4::new();
        assert_eq!(*ne4.registers.first().unwrap(), 0u16);
        assert_eq!(*ne4.registers.last().unwrap(), 0u16);
        ne4.update();
        assert_eq!(*ne4.registers.first().unwrap(), 0);
        assert_eq!(*ne4.registers.last().unwrap(), 99);
    }

    /// Test Rreg functions
    #[test]
    fn device_id() {
        let mut ne4 = NE4::new();
        ne4.registers[0] = 0x1000;
        assert_eq!(ne4.device_id(), 4096);
    }

    #[test]
    fn work_mode() {
        let mut ne4 = NE4::new();
        ne4.registers[1] = 0x1000;
        assert_eq!(ne4.work_mode(), 4096);
    }
    #[test]
    fn concentration_gas() {
        let mut ne4 = NE4::new();
        ne4.registers[2] = 0x1000;
        assert_eq!(ne4.concentration_gas(), 4096);
    }
    #[test]
    fn output_current() {
        let mut ne4 = NE4::new();
        ne4.registers[3] = 0x1000;
        assert_eq!(ne4.output_current(), 4096);
    }
    #[test]
    fn int_temp() {
        let mut ne4 = NE4::new();
        ne4.registers[4] = 0x1000;
        assert_eq!(ne4.int_temp(), 4096);
    }
    #[test]
    fn adc_temp() {
        let mut ne4 = NE4::new();
        ne4.registers[40] = 0x1000;
        assert_eq!(ne4.adc_temp(), 4096);
    }
    #[test]
    fn adc_poti() {
        let mut ne4 = NE4::new();
        ne4.registers[41] = 0x1000;
        assert_eq!(ne4.adc_poti(), 4096);
    }
    #[test]
    fn adc_sensor() {
        let mut ne4 = NE4::new();
        ne4.registers[42] = 0x1000;
        assert_eq!(ne4.adc_sensor(), 4096);
    }
    #[test]
    fn amplification_poti() {
        let mut ne4 = NE4::new();
        ne4.registers[43] = 0x1000;
        assert_eq!(ne4.amplification_poti(), 4096);
    }
    #[test]
    fn amplification_temp() {
        let mut ne4 = NE4::new();
        ne4.registers[44] = 0x1000;
        assert_eq!(ne4.amplification_temp(), 4096);
    }
    #[test]
    fn adc_sensor_corrected() {
        let mut ne4 = NE4::new();
        ne4.registers[45] = 0x1000;
        assert_eq!(ne4.adc_sensor_corrected(), 4096);
    }
    #[test]
    fn concentration_gas_calculated() {
        let mut ne4 = NE4::new();
        ne4.registers[46] = 0x1000;
        assert_eq!(ne4.concentration_gas_calculated(), 4096);
    }
    #[test]
    fn software_date() {
        let mut ne4 = NE4::new();
        ne4.registers[49] = 0x1000;
        assert_eq!(ne4.software_date(), 4096);
    }
}
