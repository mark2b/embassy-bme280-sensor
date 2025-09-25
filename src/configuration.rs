use crate::{Filter, Oversampling, SensorMode, StandbyDuration};

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

impl Configuration {
    pub(crate) fn to_low_level_configuration(
        &self,
    ) -> (Config, ControlMeasurement, ControlHumidity) {
        self.into()
    }

    /// Set the standby time
    #[must_use]
    pub fn with_standby_time(mut self, standby_time: StandbyTime) -> Self {
        self.standby_time = standby_time;
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

    /// Check if chip is in forced mode
    #[doc(hidden)]
    pub(crate) fn is_forced(&self) -> bool {
        self.sensor_mode == SensorMode::Forced
    }
}

/// Low-level config item
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Config(u8);

impl From<(StandbyTime, Filter, bool)> for Config {
    fn from((standby_time, filter, spi3w): (StandbyTime, Filter, bool)) -> Self {
        let standby_time = standby_time.to_value() & 0b111;
        let filter = filter.to_value() & 0b111;
        let spi3w = u8::from(spi3w) & 0b1;
        Self(standby_time << 5 | filter << 2 | spi3w)
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


#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
enum StandbyDuration {
    #[default]
    Standby0_5ms = 0b000,
    Standby10ms = 0b110,
    Oversample2,
    Oversample4,
    Oversample8,
    Oversample16,
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

impl crate::Oversampling {
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

impl Default for crate::Oversampling {
    fn default() -> Self {
        Self::Skip
    }
}
