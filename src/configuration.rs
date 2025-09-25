#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Configuration {
    standby_duration: StandbyDuration,
    filter: Filter,
    spi3w: bool,
    temperature_oversampling: Oversampling,
    pressure_oversampling: Oversampling,
    humidity_oversampling: Oversampling,
    sensor_mode: SensorMode,
}

impl From<&Configuration> for (Config, ControlMeasurement, ControlHumidity) {
    fn from(configuration: &Configuration) -> Self {
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

impl Configuration {
    pub(crate) fn to_low_level_configuration(
        &self,
    ) -> (Config, ControlMeasurement, ControlHumidity) {
        self.into()
    }

    /// Set the standby time
    #[must_use]
    pub fn with_standby_time(mut self, standby_duration: StandbyDuration) -> Self {
        self.standby_duration = standby_duration;
        self
    }

    /// Set the filter
    #[must_use]
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    /// Set the SPI3w option
    #[doc(hidden)]
    #[allow(unused)]
    pub(crate) fn with_spi3w(mut self, spi3w: bool) -> Self {
        self.spi3w = spi3w;
        self
    }

    /// Set the oversampling factor for temperature
    #[must_use]
    pub fn with_temperature_oversampling(mut self, temperature_oversampling: Oversampling) -> Self {
        self.temperature_oversampling = temperature_oversampling;
        self
    }

    /// Set the oversampling factor for pressure
    #[must_use]
    pub fn with_pressure_oversampling(mut self, pressure_oversampling: Oversampling) -> Self {
        self.pressure_oversampling = pressure_oversampling;
        self
    }

    /// Set the oversampling factor for humidity
    #[must_use]
    pub fn with_humidity_oversampling(mut self, humidity_oversampling: Oversampling) -> Self {
        self.humidity_oversampling = humidity_oversampling;
        self
    }

    /// Set the sensor mode
    #[must_use]
    pub fn with_sensor_mode(mut self, sensor_mode: SensorMode) -> Self {
        self.sensor_mode = sensor_mode;
        self
    }

    pub fn with_standby_duration(mut self, standby_duration: StandbyDuration) -> Self {
        self.standby_duration = standby_duration;
        self
    }

    /// Check if chip is in forced mode
    #[doc(hidden)]
    pub(crate) fn is_forced(&self) -> bool {
        self.sensor_mode == SensorMode::Forced
    }
}

/// Low-level config item
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Config(u8);

impl From<(StandbyDuration, Filter, bool)> for Config {
    fn from((standby_duration, filter, spi3w): (StandbyDuration, Filter, bool)) -> Self {
        let standby_duration = standby_duration.to_value() & 0b111;
        let filter = filter.to_value() & 0b111;
        let spi3w = u8::from(spi3w) & 0b1;
        Self(standby_duration << 5 | filter << 2 | spi3w)
    }
}

impl From<Config> for u8 {
    fn from(config: Config) -> Self {
        config.0
    }
}

/// Low-level control humidity item
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlHumidity(u8);

impl From<Oversampling> for ControlHumidity {
    fn from(humidity_oversampling: Oversampling) -> Self {
        Self(humidity_oversampling.to_value() & 0b111)
    }
}

impl From<ControlHumidity> for u8 {
    fn from(ctrl_hum: ControlHumidity) -> Self {
        ctrl_hum.0
    }
}

/// Low-level control measurement item
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
        let oversampling_temperature = oversampling_temperature.to_value() & 0b111;
        let oversampling_pressure = oversampling_pressure.to_value() & 0b111;
        let sensor_mode = sensor_mode.to_value() & 0b11;
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
    Millis0_5,
    Millis10,
    Millis20,
    Millis62_5,
    Millis125,
    Millis250,
    Millis500,
    Millis1000,
}

impl StandbyDuration {
    pub(crate) fn to_value(self) -> u8 {
        match self {
            Self::Millis0_5 => 0b000,
            Self::Millis10 => 0b110,
            Self::Millis20 => 0b111,
            Self::Millis62_5 => 0b001,
            Self::Millis125 => 0b010,
            Self::Millis250 => 0b011,
            Self::Millis500 => 0b100,
            Self::Millis1000 => 0b101,
        }
    }
}

/// Oversampling setting
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Oversampling {
    /// Skip the measurement altogether
    Skip,
    /// Take a single sample
    Oversample1,
    /// Take two samples
    Oversample2,
    /// Take four samples
    Oversample4,
    /// Take eight samples
    Oversample8,
    /// Take sixteen samples
    Oversample16,
}

impl Oversampling {
    /// Convert to binary value
    pub(crate) fn to_value(self) -> u8 {
        match self {
            Self::Skip => 0b000,
            Self::Oversample1 => 0b001,
            Self::Oversample2 => 0b010,
            Self::Oversample4 => 0b011,
            Self::Oversample8 => 0b100,
            Self::Oversample16 => 0b101,
        }
    }
}

impl Default for Oversampling {
    fn default() -> Self {
        Self::Skip
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SensorMode {
    Sleep,
    Forced,
    Normal,
}
impl From<SensorMode> for u8 {
    fn from(mode: SensorMode) -> Self {
        mode.to_value()
    }
}

impl SensorMode {
    /// Convert to binary value
    #[doc(hidden)]
    pub(crate) fn to_value(self) -> u8 {
        match self {
            Self::Sleep => 0b00,
            Self::Forced => 0b01,
            Self::Normal => 0b11,
        }
    }
}

impl Default for SensorMode {
    fn default() -> Self {
        Self::Sleep
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum Filter {
    #[default]
    Off = 0b000,
    FilterX2 = 0b001,
    FilterX4 = 0b010,
    FilterX8 = 0b011,
    FilterX16 = 0b100,
}

impl Filter {
    pub(crate) fn to_value(self) -> u8 {
        match self {
            Self::Off => 0b000,
            Self::FilterX2 => 0b001,
            Self::FilterX4 => 0b010,
            Self::FilterX8 => 0b011,
            Self::FilterX16 => 0b100,
        }
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
enum SensorSampling {
    #[default]
    SamplingNone = 0b000,
    SamplingX1 = 0b001,
    SamplingX2 = 0b010,
    SamplingX4 = 0b011,
    SamplingX8 = 0b100,
    SamplingX16 = 0b101,
}
