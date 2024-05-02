#[derive(Debug, Clone)]
pub struct SpoilerPosition {
    l1: f64,
    l2: f64,
    l3: f64,
    l4: f64,
    l5: f64,
    r1: f64,
    r2: f64,
    r3: f64,
    r4: f64,
    r5: f64,
}

#[derive(Debug, Clone)]
pub struct SpoilerController {
    yoke_roll_position_f64: f64,
    on_ground: bool,
    speed_brake_handle_poition_0_to_100: f64,
    in_landing_config: bool, // we will look for the first instance of touching down in the landing configuration then deploy the GS
    spoiler_position: SpoilerPosition,
}

impl SpoilerController {
    pub fn new(
        yoke_roll_position_f64: f64,
        on_ground: bool,
        speed_brake_handle_poition_0_to_100: f64,
        in_landing_config: bool,
    ) -> Self {
        Self {
            yoke_roll_position_f64,
            on_ground,
            speed_brake_handle_poition_0_to_100,
            in_landing_config,
            spoiler_position: SpoilerPosition {
                l1: 0.,
                l2: 0.,
                l3: 0.,
                l4: 0.,
                l5: 0.,
                r1: 0.,
                r2: 0.,
                r3: 0.,
                r4: 0.,
                r5: 0.,
            },
        }
    }

    pub fn update_input(
        &mut self,
        yoke_roll_position_f64: f64,
        on_ground: bool,
        speed_brake_handle_poition_0_to_100: f64,
    ) {
        self.yoke_roll_position_f64 = yoke_roll_position_f64;
        self.on_ground = on_ground;
        self.speed_brake_handle_poition_0_to_100 = speed_brake_handle_poition_0_to_100
    }

    fn spoiler_handle_to_demanded_position(handle_position: f64) -> f64 {
        let clamped = handle_position.max(0.0).min(100.0);

        let output = (clamped / 100.0) * 60.0;

        output
    }

    pub fn get_demanded_spoiler_position(&mut self) -> SpoilerPosition {
        static mut FIRST_TOUCHDOWN_TRACKER: bool = false;
        // need to research some more into the conditions that would cause the ground spoilers to deploy automatically the in_landing_config will be written elsewehere to actually determine if the spoilers should deploy upon touching the ground
        match self.in_landing_config {
            true => match self.on_ground {
                true => todo!(),
                false => todo!(),
            }, // auto deploy
            false => match self.on_ground {
                true => {
                    let demanded_spoiler_position = Self::spoiler_handle_to_demanded_position(
                        self.speed_brake_handle_poition_0_to_100,
                    );
                    // the ground spoilers dont deploy with the spoiler handle apperently
                    let updated_spoiler = SpoilerPosition {
                        l1: 0.,
                        l2: 0.,
                        l3: demanded_spoiler_position,
                        l4: demanded_spoiler_position,
                        l5: demanded_spoiler_position,
                        r1: 0.,
                        r2: 0.,
                        r3: demanded_spoiler_position,
                        r4: demanded_spoiler_position,
                        r5: demanded_spoiler_position,
                    };
                    self.spoiler_position = updated_spoiler.clone();
                    updated_spoiler
                }
                false => {
                    // need to probe the handle position to make sure its at 0 (above 0 will be manual deploy, at 0 will be with ailerons)
                    if self.speed_brake_handle_poition_0_to_100 == 0. {
                        let yoke_pos = self.yoke_roll_position_f64.is_sign_positive();
                        let demanded_spoiler_pos_left;
                        let demanded_spoiler_pos_right;
                        if yoke_pos {
                            // poositive is right yoke deflection,
                            demanded_spoiler_pos_right = self.yoke_roll_position_f64.abs();
                            demanded_spoiler_pos_left = 0.0;
                        } else {
                            demanded_spoiler_pos_left = self.yoke_roll_position_f64.abs();
                            demanded_spoiler_pos_right = 0.0;
                        }

                        let updated_spoiler = SpoilerPosition {
                            l1: 0.,
                            l2: 0.,
                            l3: demanded_spoiler_pos_left,
                            l4: demanded_spoiler_pos_left,
                            l5: demanded_spoiler_pos_left,
                            r1: 0.,
                            r2: 0.,
                            r3: demanded_spoiler_pos_right,
                            r4: demanded_spoiler_pos_right,
                            r5: demanded_spoiler_pos_right,
                        };
                        self.spoiler_position = updated_spoiler.clone();
                        updated_spoiler
                    } else {
                        let demanded_spoiler_position = Self::spoiler_handle_to_demanded_position(
                            self.speed_brake_handle_poition_0_to_100,
                        );
                        // the ground spoilers dont deploy with the spoiler handle apperently
                        let updated_spoiler = SpoilerPosition {
                            l1: 0.,
                            l2: 0.,
                            l3: demanded_spoiler_position,
                            l4: demanded_spoiler_position,
                            l5: demanded_spoiler_position,
                            r1: 0.,
                            r2: 0.,
                            r3: demanded_spoiler_position,
                            r4: demanded_spoiler_position,
                            r5: demanded_spoiler_position,
                        };
                        self.spoiler_position = updated_spoiler.clone();
                        updated_spoiler
                    }
                }
            }, // manual deploy / assist ailerons
        }
    }
}
