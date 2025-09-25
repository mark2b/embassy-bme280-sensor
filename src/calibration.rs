pub struct CalibrationRegisters {
    pub dig_t1: u16,
    pub dig_t2: i16,
    pub dig_t3: i16,
    pub dig_p1: u16,
    pub dig_p2: i16,
    pub dig_p3: i16,
    pub dig_p4: i16,
    pub dig_p5: i16,
    pub dig_p6: i16,
    pub dig_p7: i16,
    pub dig_p8: i16,
    pub dig_p9: i16,
    pub dig_h1: u8,
    pub dig_h2: i16,
    pub dig_h3: u8,
    pub dig_h4: i16,
    pub dig_h5: i16,
    pub dig_h6: i8,
}

impl CalibrationRegisters {
    pub fn compensate_temperature(&self, adc_t: i32) -> i32 {
        let var1 = (((adc_t >> 3) - (i32::from(self.dig_t1) << 1)) * i32::from(self.dig_t2)) >> 11;
        let var2 = (((((adc_t >> 4) - i32::from(self.dig_t1))
            * ((adc_t >> 4) - i32::from(self.dig_t1)))
            >> 12)
            * i32::from(self.dig_t3))
            >> 14;

        var1 + var2
    }

    pub fn compensate_humidity(&self, adc_h: u16, t_fine: i32) -> u32 {
        let adc_h = i32::from(adc_h);

        let v_x1_u32r: i32 = t_fine - 76_800_i32;
        let v_x1_u32r: i32 = ((((adc_h << 14)
            - (i32::from(self.dig_h4) << 20)
            - (i32::from(self.dig_h5) * v_x1_u32r))
            + (16_384_i32))
            >> 15)
            * (((((((v_x1_u32r * i32::from(self.dig_h6)) >> 10)
                * (((v_x1_u32r * i32::from(self.dig_h3)) >> 11) + (32_768_i32)))
                >> 10)
                + (2_097_152_i32))
                * i32::from(self.dig_h2)
                + 8192_i32)
                >> 14);
        let v_x1_u32r: i32 = v_x1_u32r
            - (((((v_x1_u32r >> 15) * (v_x1_u32r >> 15)) >> 7) * i32::from(self.dig_h1)) >> 4);
        let v_x1_u32r = if v_x1_u32r < 0 { 0 } else { v_x1_u32r };
        let v_x1_u32r = if v_x1_u32r > 419_430_400 {
            419_430_400
        } else {
            v_x1_u32r
        };

        let humidity = v_x1_u32r >> 12;
        humidity as u32
    }

    pub fn compensate_pressure(&self, adc_p: u32, t_fine: i32) -> u32 {
        let var1 = i64::from(t_fine) - 128_000;
        let var2 = var1 * var1 * i64::from(self.dig_p6);
        let var2 = var2 + ((var1 * i64::from(self.dig_p5)) << 17);
        let var2 = var2 + (i64::from(self.dig_p4) << 35);
        let var1 =
            ((var1 * var1 * i64::from(self.dig_p3)) >> 8) + ((var1 * i64::from(self.dig_p2)) << 12);
        let var1 = ((((1_i64) << 47) + var1) * i64::from(self.dig_p1)) >> 33;

        if var1 == 0 {
            0
        } else {
            let var4 = 1_048_576 - i64::from(adc_p);
            let var4 = (((var4 << 31) - var2) * 3125) / var1;
            let var1 = (i64::from(self.dig_p9) * (var4 >> 13) * (var4 >> 13)) >> 25;
            let var2 = (i64::from(self.dig_p8) * var4) >> 19;
            let var5 = ((var4 + var1 + var2) >> 8) + (i64::from(self.dig_p7) << 4);

            let p = var5;
            let pressure = p as u32;

            pressure
        }
    }
}
