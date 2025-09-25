#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SamplingConfiguration {
    standby_duration: StandbyDuration,
    filter: Filter,
    spi3w: bool,
    temperature_oversampling: Oversampling,
    pressure_oversampling: Oversampling,
    humidity_oversampling: Oversampling,
    sensor_mode: SensorMode,
}

impl From<&SamplingConfiguration> for (Config, ControlMeasurement, ControlHumidity) {
    fn from(configuration: &SamplingConfiguration) -> Self {
        let config = (
            configuration.standby_duration.clone(),
            configuration.filter.clone(),
            configuration.spi3w,
        )
            .into();
        let control_measurement = (
            configuration.temperature_oversampling,
            configuration.pressure_oversampling,
            configuration.sensor_mode,
        )
            .into();
        let control_humidity = configuration.humidity_oversampling.into();
        (config, control_measurement, control_humidity)
    }
}

impl SamplingConfiguration {
    pub(crate) fn to_low_level_configuration(
        &self,
    ) -> (Config, ControlMeasurement, ControlHumidity) {
        self.into()
    }

    pub fn with_standby_time(mut self, standby_duration: StandbyDuration) -> Self {
        self.standby_duration = standby_duration;
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    #[allow(unused)]
    pub(crate) fn with_spi3w(mut self, spi3w: bool) -> Self {
        self.spi3w = spi3w;
        self
    }

    pub fn with_temperature_oversampling(mut self, temperature_oversampling: Oversampling) -> Self {
        self.temperature_oversampling = temperature_oversampling;
        self
    }

    pub fn with_pressure_oversampling(mut self, pressure_oversampling: Oversampling) -> Self {
        self.pressure_oversampling = pressure_oversampling;
        self
    }

    pub fn with_humidity_oversampling(mut self, humidity_oversampling: Oversampling) -> Self {
        self.humidity_oversampling = humidity_oversampling;
        self
    }

    pub fn with_sensor_mode(mut self, sensor_mode: SensorMode) -> Self {
        self.sensor_mode = sensor_mode;
        self
    }

    pub fn with_standby_duration(mut self, standby_duration: StandbyDuration) -> Self {
        self.standby_duration = standby_duration;
        self
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Config(u8);

impl From<(StandbyDuration, Filter, bool)> for Config {
    fn from((standby_duration, filter, spi3w): (StandbyDuration, Filter, bool)) -> Self {
        let standby_duration = (standby_duration as u8) & 0b111;
        let filter = (filter as u8) & 0b111;
        let spi3w = u8::from(spi3w) & 0b1;
        Self(standby_duration << 5 | filter << 2 | spi3w)
    }
}

impl From<Config> for u8 {
    fn from(config: Config) -> Self {
        config.0
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlHumidity(u8);

impl From<Oversampling> for ControlHumidity {
    fn from(humidity_oversampling: Oversampling) -> Self {
        Self((humidity_oversampling as u8) & 0b111)
    }
}

impl From<ControlHumidity> for u8 {
    fn from(ctrl_hum: ControlHumidity) -> Self {
        ctrl_hum.0
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlMeasurement(u8);

impl From<(Oversampling, Oversampling, SensorMode)> for ControlMeasurement {
    fn from(
        (oversampling_temperature, oversampling_pressure, sensor_mode): (
            Oversampling,
            Oversampling,
            SensorMode,
        ),
    ) -> Self {
        let oversampling_temperature = (oversampling_temperature as u8) & 0b111;
        let oversampling_pressure = (oversampling_pressure as u8) & 0b111;
        let sensor_mode = (sensor_mode as u8) & 0b11;
        Self(oversampling_temperature << 5 | oversampling_pressure << 2 | sensor_mode)
    }
}

impl From<ControlMeasurement> for u8 {
    fn from(ctrl_meas: ControlMeasurement) -> Self {
        ctrl_meas.0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum StandbyDuration {
    #[default]
    Millis0_5 = 0b000,
    Millis10 = 0b110,
    Millis20 = 0b111,
    Millis62_5 = 0b001,
    Millis125 = 0b010,
    Millis250 = 0b011,
    Millis500 = 0b100,
    Millis1000 = 0b101,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Oversampling {
    Skip = 0b000,
    #[default]
    X1 = 0b001,
    X2 = 0b010,
    X4 = 0b011,
    X8 = 0b100,
    X16 = 0b101,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum SensorMode {
    #[default]
    Sleep = 0b00,
    Forced = 0b01,
    Normal = 0b11,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum Filter {
    #[default]
    Off = 0b000,
    X2 = 0b001,
    X4 = 0b010,
    X8 = 0b011,
    X16 = 0b100,
}
