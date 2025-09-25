#![no_std]
#![no_main]

use defmt::Format;
use crate::configuration::StandbyDuration;

pub mod bme280_rp;
pub mod configuration;
pub mod calibration;

const BME280_REGISTER_CHIPID: u8 = 0xD0;
const BME280_REGISTER_SOFTRESET: u8 = 0xE0;
const BME280_REGISTER_STATUS: u8 = 0xF3;

const BME280_REGISTER_DIG_FIRST_START: u8 = BME280_REGISTER_DIG_T1;
const BME280_REGISTER_DIG_FIRST_END: u8 = BME280_REGISTER_DIG_H1 + size_of::<u8>() as u8;
const BME280_REGISTER_DIG_FIRST_LENGTH: usize =
    (BME280_REGISTER_DIG_FIRST_END - BME280_REGISTER_DIG_FIRST_START) as usize;

const BME280_REGISTER_DIG_SECOND_START: u8 = BME280_REGISTER_DIG_H2;
const BME280_REGISTER_DIG_SECOND_END: u8 = BME280_REGISTER_DIG_H6 + size_of::<u8>() as u8;
const BME280_REGISTER_DIG_SECOND_LENGTH: usize =
    (BME280_REGISTER_DIG_SECOND_END - BME280_REGISTER_DIG_SECOND_START) as usize;

const BME280_REGISTER_DIG_T1: u8 = 0x88;
const BME280_REGISTER_DIG_T2: u8 = 0x8A;
const BME280_REGISTER_DIG_T3: u8 = 0x8C;
const BME280_REGISTER_DIG_P1: u8 = 0x8E;
const BME280_REGISTER_DIG_P2: u8 = 0x90;
const BME280_REGISTER_DIG_P3: u8 = 0x92;
const BME280_REGISTER_DIG_P4: u8 = 0x94;
const BME280_REGISTER_DIG_P5: u8 = 0x96;
const BME280_REGISTER_DIG_P6: u8 = 0x98;
const BME280_REGISTER_DIG_P7: u8 = 0x9A;
const BME280_REGISTER_DIG_P8: u8 = 0x9C;
const BME280_REGISTER_DIG_P9: u8 = 0x9E;
const BME280_REGISTER_DIG_H1: u8 = 0xA1;
const BME280_REGISTER_DIG_H2: u8 = 0xE1;
const BME280_REGISTER_DIG_H3: u8 = 0xE3;
const BME280_REGISTER_DIG_H4: u8 = 0xE4;
const BME280_REGISTER_DIG_H5: u8 = 0xE5;
const BME280_REGISTER_DIG_H6: u8 = 0xE7;

const BME280_REGISTER_CAL26: u8 = 0xE1;
const BME280_REGISTER_CONTROLHUMID: u8 = 0xF2;
const BME280_REGISTER_CONTROL: u8 = 0xF4;
const BME280_REGISTER_CONFIG: u8 = 0xF5;
const BME280_REGISTER_PRESSUREDATA: u8 = 0xF7;
const BME280_REGISTER_TEMPDATA: u8 = 0xFA;
const BME280_REGISTER_HUMIDDATA: u8 = 0xFD;

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
}
