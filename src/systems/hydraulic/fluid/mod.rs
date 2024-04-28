#[derive(Debug, Clone)]
pub struct Viscosity {
    pub c_st_minus_54c: f64,
    pub c_st_minus_40c: f64,
    pub c_st_40c: f64,
    pub c_st_100c: f64,
}

#[derive(Debug, Clone)]
pub struct ParticulateContamination {
    pub size_5_15_microns: u32,
    pub size_16_25_microns: u32,
    pub size_26_50_microns: u32,
    pub size_51_100_microns: u32,
    pub size_100_plus_microns: u32,
}

#[derive(Debug, Clone)]
pub struct HydraulicFluid {
    pub iso_grade: u8,
    pub gravity_api: f64,
    pub bulk_modulus: f64,
    pub specific_gravity_60f: f64,
    pub density_kg_m_3_60f: f64,
    pub color: String,
    pub flash_point_c: f64,
    pub pour_point_c: f64,
    pub viscosity_cst: Viscosity,
    pub viscosity_index: u16,
    pub acid_number: f64,
    pub copper_corrosion_rating: String,
    pub dielectric_strength_kv: f64,
    pub evaporation_loss_pct: f64,
    pub foam_resistance: String,
    pub four_ball_wear_scar_diameter_mm: f64,
    pub gravimetric_filtration_mg_100ml: f64,
    pub filter_time_minutes: u8,
    pub particulate_contamination: ParticulateContamination,
    pub water_content_ppm: u32,
}

impl HydraulicFluid {
    pub fn new() -> Self {
        Self {
            iso_grade: 15,
            gravity_api: 31.0,
            bulk_modulus: 1e-9,
            specific_gravity_60f: 0.871,
            density_kg_m_3_60f: 119.826,
            color: String::from("Red"),
            flash_point_c: 90.0,
            pour_point_c: -64.0,
            viscosity_cst: Viscosity {
                c_st_minus_54c: 2450.0,
                c_st_minus_40c: 495.0,
                c_st_40c: 13.5,
                c_st_100c: 5.1,
            },
            viscosity_index: 382,
            acid_number: 0.05,
            copper_corrosion_rating: String::from("1b"),
            dielectric_strength_kv: 49.6,
            evaporation_loss_pct: 13.6,
            foam_resistance: String::from("25-0"),
            four_ball_wear_scar_diameter_mm: 0.65,
            gravimetric_filtration_mg_100ml: 6.0,
            filter_time_minutes: 6,
            particulate_contamination: ParticulateContamination {
                size_5_15_microns: 1200,
                size_16_25_microns: 175,
                size_26_50_microns: 60,
                size_51_100_microns: 5,
                size_100_plus_microns: 0,
            },
            water_content_ppm: 60,
        }
    }
    pub fn get_viscosity(&self, temperature_c: f64) -> f64 {
        if temperature_c <= -54. {
            self.viscosity_cst.c_st_minus_54c
        } else if temperature_c <= -40. {
            let interpoolate = (self.viscosity_cst.c_st_minus_40c
                - self.viscosity_cst.c_st_minus_54c)
                / (-40. - (-54.));
            self.viscosity_cst.c_st_minus_54c + interpoolate * (temperature_c - (-54.))
        } else if temperature_c <= 40. {
            let interpolate =
                (self.viscosity_cst.c_st_40c - self.viscosity_cst.c_st_minus_40c) / (40. - (-40.));
            self.viscosity_cst.c_st_minus_40c + interpolate * (temperature_c - (-40.0))
        } else {
            let interpolate =
                (self.viscosity_cst.c_st_100c - self.viscosity_cst.c_st_40c) / (100. - 40.);
            self.viscosity_cst.c_st_40c + interpolate * (temperature_c - 40.)
        }
    }
    pub fn get_volume_change_from_compression_percent(
        &self,
        pressure_psi: f64,
        volume: f64,
    ) -> f64 {
        let delta_v = volume * self.bulk_modulus * pressure_psi;
        (delta_v / volume) * 100.
    }
}
