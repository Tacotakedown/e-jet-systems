#[derive(Debug)]
struct Viscosity {
    c_st_minus_54c: f64,
    c_st_minus_40c: f64,
    c_st_40c: f64,
    c_st_100c: f64,
}

#[derive(Debug)]
struct ParticulateContamination {
    size_5_15_microns: u32,
    size_16_25_microns: u32,
    size_26_50_microns: u32,
    size_51_100_microns: u32,
    size_100_plus_microns: u32,
}

#[derive(Debug)]
pub struct HydraulicFluid {
    iso_grade: u8,
    gravity_api: f64,
    specific_gravity_60f: f64,
    density_lbs_gal_60f: f64,
    color: String,
    flash_point_c: f64,
    pour_point_c: f64,
    viscosity_cst: Viscosity,
    viscosity_index: u16,
    acid_number: f64,
    copper_corrosion_rating: String,
    dielectric_strength_kv: f64,
    evaporation_loss_pct: f64,
    foam_resistance: String,
    four_ball_wear_scar_diameter_mm: f64,
    gravimetric_filtration_mg_100ml: f64,
    filter_time_minutes: u8,
    particulate_contamination: ParticulateContamination,
    water_content_ppm: u32,
}

impl HydraulicFluid {
    pub fn new() -> Self {
        Self {
            iso_grade: 15,
            gravity_api: 31.0,
            specific_gravity_60f: 0.871,
            density_lbs_gal_60f: 7.25,
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
}
