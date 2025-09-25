use crate::calibration::CalibrationData;
use crate::configuration::{Configuration, Filter, Oversampling, SensorMode, StandbyDuration};
use crate::BME280Error::{InvalidData, NoData};
use crate::{
    BME280Error, BME280Response, BME280_REGISTER_CHIPID,
    BME280_REGISTER_CONFIG, BME280_REGISTER_CONTROL, BME280_REGISTER_CONTROLHUMID,
    BME280_REGISTER_DIG_FIRST_END, BME280_REGISTER_DIG_FIRST_LENGTH, BME280_REGISTER_DIG_FIRST_START,
    BME280_REGISTER_DIG_H1, BME280_REGISTER_DIG_H2, BME280_REGISTER_DIG_H3, BME280_REGISTER_DIG_H4,
    BME280_REGISTER_DIG_H5, BME280_REGISTER_DIG_H6, BME280_REGISTER_DIG_P1, BME280_REGISTER_DIG_P2,
    BME280_REGISTER_DIG_P3, BME280_REGISTER_DIG_P4, BME280_REGISTER_DIG_P5, BME280_REGISTER_DIG_P6,
    BME280_REGISTER_DIG_P7, BME280_REGISTER_DIG_P8, BME280_REGISTER_DIG_P9,
    BME280_REGISTER_DIG_SECOND_END, BME280_REGISTER_DIG_SECOND_LENGTH, BME280_REGISTER_DIG_SECOND_START,
    BME280_REGISTER_DIG_T1, BME280_REGISTER_DIG_T2, BME280_REGISTER_DIG_T3,
    BME280_REGISTER_PRESSUREDATA, BME280_REGISTER_SOFTRESET, BME280_REGISTER_STATUS, BME280_REGISTER_TEMPDATA,
};
use core::cmp::PartialEq;
use defmt::info;
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;

macro_rules! concat_bytes {
    ($msb:expr, $lsb:expr) => {
        (($msb as u16) << 8) | ($lsb as u16)
    };
}

pub struct BME280Sensor<'a, T: I2c> {
    i2c: &'a mut T,
    address: u8,
    last_response: Option<BME280Response>,
    last_read_time: Option<embassy_time::Instant>,
    calibration_data: Option<CalibrationData>,
    configuration: Configuration,
}

impl<'a, T: I2c> BME280Sensor<'a, T> {
    pub fn new(i2c: &'a mut T, address: u8) -> Self {
        Self {
            i2c,
            address,
            last_response: None,
            last_read_time: None,
            calibration_data: None,
            configuration: Configuration::default(),
        }
    }

    pub async fn setup(&mut self) -> Result<(), BME280Error> {
        let chip_id = self.read_register_8bit(BME280_REGISTER_CHIPID).await?;
        if chip_id != 0x60 {
            return Err(BME280Error::InvalidChipId(chip_id));
        }
        self.write_register_8bit(BME280_REGISTER_SOFTRESET, 0x86)
            .await?;
        Timer::after(embassy_time::Duration::from_millis(10)).await;
        while self.is_reading_calibration().await? {
            Timer::after(embassy_time::Duration::from_millis(10)).await;
        }
        self.read_coefficients().await?;
        self.set_sampling_configuration(
            Configuration::default()
                .with_temperature_oversampling(Oversampling::Oversample2)
                .with_pressure_oversampling(Oversampling::Oversample2)
                .with_humidity_oversampling(Oversampling::Oversample2)
                .with_sensor_mode(SensorMode::Normal)
                .with_standby_duration(StandbyDuration::Millis1000)
                .with_filter(Filter::FilterX2),
        )
        .await?;
        Timer::after(embassy_time::Duration::from_millis(100)).await;
        Ok(())
    }

    async fn is_reading_calibration(&mut self) -> Result<bool, BME280Error> {
        let status = self.read_register_8bit(BME280_REGISTER_STATUS).await?;
        Ok((status & (1 << 3)) != 0)
    }

    async fn read_coefficients(&mut self) -> Result<(), BME280Error> {
        let mut data = [0u8; BME280_REGISTER_DIG_FIRST_LENGTH + BME280_REGISTER_DIG_SECOND_LENGTH];
        self.read_raw(0x88, &mut data[0..BME280_REGISTER_DIG_FIRST_LENGTH])
            .await?;
        self.read_raw(
            0xE1,
            &mut data[BME280_REGISTER_DIG_FIRST_LENGTH
                ..BME280_REGISTER_DIG_FIRST_LENGTH + BME280_REGISTER_DIG_SECOND_LENGTH],
        )
        .await?;

        self.calibration_data = Some(CalibrationData {
            dig_t1: u16::from_le_bytes([data[0], data[1]]),
            dig_t2: i16::from_le_bytes([data[2], data[3]]),
            dig_t3: i16::from_le_bytes([data[4], data[5]]),
            dig_p1: u16::from_le_bytes([data[6], data[7]]),
            dig_p2: i16::from_le_bytes([data[8], data[9]]),
            dig_p3: i16::from_le_bytes([data[10], data[11]]),
            dig_p4: i16::from_le_bytes([data[12], data[13]]),
            dig_p5: i16::from_le_bytes([data[14], data[15]]),
            dig_p6: i16::from_le_bytes([data[16], data[17]]),
            dig_p7: i16::from_le_bytes([data[18], data[19]]),
            dig_p8: i16::from_le_bytes([data[20], data[21]]),
            dig_p9: i16::from_le_bytes([data[22], data[23]]),
            dig_h1: data[25],
            dig_h2: i16::from_le_bytes([data[26], data[27]]),
            dig_h3: data[28],
            dig_h4: i16::from(data[29]) << 4 | i16::from(data[30]) & 0xf,
            dig_h5: ((i16::from(data[30]) & 0xf0) >> 4) | (i16::from(data[31]) << 4),
            dig_h6: data[32] as i8,
        });

        Ok(())
    }

    async fn set_sampling_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), BME280Error> {
        self.configuration = configuration;

        let (config, ctrl_meas, ctrl_hum) = self.configuration.to_low_level_configuration();

        self.write_register_8bit(BME280_REGISTER_CONTROL, SensorMode::Sleep.into())
            .await?;
        self.write_register_8bit(BME280_REGISTER_CONTROLHUMID, ctrl_hum.into())
            .await?;
        self.write_register_8bit(BME280_REGISTER_CONFIG, config.into())
            .await?;
        self.write_register_8bit(BME280_REGISTER_CONTROL, ctrl_meas.into())
            .await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<BME280Response, BME280Error> {
        let mut data: [u8; 8] = [0; 8];
        self.read_raw(BME280_REGISTER_PRESSUREDATA, &mut data)
            .await?;

        let data_msb = (data[0] as u32) << 12;
        let data_lsb = (data[1] as u32) << 4;
        let data_xlsb = (data[2] as u32) >> 4;
        let adc_p = data_msb | data_lsb | data_xlsb;

        let data_msb = (data[3] as u32) << 12;
        let data_lsb = (data[4] as u32) << 4;
        let data_xlsb = (data[5] as u32) >> 4;
        let adc_t = (data_msb | data_lsb | data_xlsb) as i32;

        let data_msb = (data[6] as u32) << 8;
        let data_lsb = data[7] as u32;
        let adc_h = data_msb | data_lsb;

        let cd = self.calibration_data.as_ref().unwrap();

        let t_fine = cd.compensate_temperature(adc_t);
        let temperature = ((t_fine * 5 + 128) >> 8) as f32 / 100.0;
        let humidity = cd.compensate_humidity(adc_h as u16, t_fine) as f32 / 1024.0;
        let pressure = cd.compensate_pressure(adc_p, t_fine) as f32 / 256.0;

        Ok(BME280Response {
            temperature,
            humidity,
            pressure,
        })
    }

    async fn read_register_8bit(&mut self, register: u8) -> Result<u8, BME280Error> {
        let mut buf = [0u8; 1];
        self.i2c_write_read(&[register], &mut buf).await?;
        Ok(buf[0])
    }

    async fn read_register_16bit(&mut self, register: u8) -> Result<u16, BME280Error> {
        let mut buf = [0u8; 2];
        self.i2c_write_read(&[register], &mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }

    async fn read_register_16bit_le(&mut self, register: u8) -> Result<u16, BME280Error> {
        let mut buf = [0u8; 2];
        self.i2c_write_read(&[register], &mut buf).await?;
        Ok(u16::from_le_bytes(buf))
    }

    async fn read_raw_pth(&mut self) -> Result<u64, BME280Error> {
        let mut buf = [0u8; 3 + 3 + 2];
        self.i2c_write_read(&[BME280_REGISTER_PRESSUREDATA], &mut buf)
            .await?;
        Ok(u64::from_be_bytes(buf))
    }

    async fn read_register_24bit(&mut self, register: u8) -> Result<u32, BME280Error> {
        let mut buf = [0u8; 4];
        self.i2c_write_read(&[register], &mut buf).await?;
        Ok(u32::from_be_bytes(buf))
    }

    async fn write_register_8bit(&mut self, register: u8, data: u8) -> Result<(), BME280Error> {
        self.i2c_write(&[register, data]).await?;
        Ok(())
    }

    async fn read_raw(&mut self, register: u8, read: &mut [u8]) -> Result<(), BME280Error> {
        self.i2c_write_read(&[register], read).await?;
        Ok(())
    }

    async fn i2c_write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), BME280Error> {
        match self.i2c.write_read(self.address, write, read).await {
            Ok(_) => Ok(()),
            Err(e) => Err(BME280Error::I2CError),
        }
    }

    async fn i2c_write(&mut self, write: &[u8]) -> Result<(), BME280Error> {
        match self.i2c.write(self.address, write).await {
            Ok(_) => Ok(()),
            Err(e) => Err(BME280Error::I2CError),
        }
    }
}
