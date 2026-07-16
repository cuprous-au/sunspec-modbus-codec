use heapless::String;

pub type Model64414 = DerSimControls;

/// Configuration parameters for the DER device simulator.
pub struct DerSimControls {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    time: Option<String<20>>,
    /// Ambient temperature (degrees Celsius)
    temperature: Option<f32>,
    /// The data source for the grid model. 'csv' or 'const'
    grid_model_source: Option<String<64>>,
    /// The data source for the irradiance model. 'csv' or 'const'
    irradiance_model_source: Option<String<64>>,
    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    irradiance: Option<f32>,
    /// Phase A RMS Voltage (pu) for the 'const' grid model
    grid_voltage_a: Option<f32>,
    /// Phase B RMS Voltage (pu) for the 'const' grid model
    grid_voltage_b: Option<f32>,
    /// Phase C RMS Voltage (pu) for the 'const' grid model
    grid_voltage_c: Option<f32>,
    /// Grid frequency (Hz) for the 'const' grid model
    grid_frequency: Option<f32>,
}

trait DerSimControlsTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    fn time(&self) -> Option<String<20>> {
        None
    }

    /// Ambient temperature (degrees Celsius)
    fn temperature(&self) -> Option<f32> {
        None
    }

    /// Ambient temperature (degrees Celsius)
    fn set_temperature(&mut self, value: f32) {}

    /// The data source for the grid model. 'csv' or 'const'
    fn grid_model_source(&self) -> Option<String<64>> {
        None
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn set_grid_model_source(&mut self, value: String<64>) {}

    /// The data source for the irradiance model. 'csv' or 'const'
    fn irradiance_model_source(&self) -> Option<String<64>> {
        None
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn set_irradiance_model_source(&mut self, value: String<64>) {}

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn irradiance(&self) -> Option<f32> {
        None
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn set_irradiance(&mut self, value: f32) {}

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_a(&self) -> Option<f32> {
        None
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_a(&mut self, value: f32) {}

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_b(&self) -> Option<f32> {
        None
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_b(&mut self, value: f32) {}

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_c(&self) -> Option<f32> {
        None
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_c(&mut self, value: f32) {}

    /// Grid frequency (Hz) for the 'const' grid model
    fn grid_frequency(&self) -> Option<f32> {
        None
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn set_grid_frequency(&mut self, value: f32) {}
}
