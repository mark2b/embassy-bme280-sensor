#![no_std]
#![no_main]

pub mod bme280_rp;
pub mod calibration;
pub mod configuration;

const MIN_REQUEST_INTERVAL_SECS: u64 = 1;

const BME280_REGISTER_CHIPID: u8 = 0xD0;
const BME280_REGISTER_SOFTRESET: u8 = 0xE0;
const BME280_REGISTER_STATUS: u8 = 0xF3;

const BME280_REGISTER_DIG_FIRST_START: u8 = 0x88;
const BME280_REGISTER_DIG_FIRST_END: u8 = 0xA1 + size_of::<u8>() as u8;
const BME280_REGISTER_DIG_FIRST_LENGTH: usize =
    (BME280_REGISTER_DIG_FIRST_END - BME280_REGISTER_DIG_FIRST_START) as usize;

const BME280_REGISTER_DIG_SECOND_START: u8 = 0xE1;
const BME280_REGISTER_DIG_SECOND_END: u8 = 0xE7 + size_of::<u8>() as u8;
const BME280_REGISTER_DIG_SECOND_LENGTH: usize =
    (BME280_REGISTER_DIG_SECOND_END - BME280_REGISTER_DIG_SECOND_START) as usize;

const BME280_REGISTER_CONTROLHUMID: u8 = 0xF2;
const BME280_REGISTER_CONTROL: u8 = 0xF4;
const BME280_REGISTER_CONFIG: u8 = 0xF5;

const BME280_REGISTER_DATA_START: u8 = 0xF7;
const BME280_REGISTER_DATA_LENGTH: usize = 8;

#[derive(Clone)]
pub struct BME280Response {
    pub humidity: f32,
    pub temperature: f32,
    pub pressure: f32,
}

#[derive(Debug, Clone)]
pub enum BME280Error {
    NoData,
    ChecksumError,
    InvalidData,
    I2CError,
    InvalidChipId(u8),
    Timeout,
    NotCalibrated,
}
