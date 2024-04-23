#[derive(Debug, Clone, Copy)]
struct AileronInterpolation {
    yoke_input: f64,
    left_aileron: f64,
    right_aileron: f64,
}

// CONTROL YOKE   |  LEFT AILERON   |  RIGHT AILERON
//  40° (LEFT)    |    25° (UP)     |    15° (DOWN)
//  35° (LEFT)    |    20.64° (UP)  |    13.54° (DOWN)
//  30° (LEFT)    |    17.01° (UP)  |    11.84° (DOWN)
//  25° (LEFT)    |    13.59° (UP)  |    10.06° (DOWN)
//  20° (LEFT)    |    10.52° (UP)  |    8.20° (DOWN)
//  15° (LEFT)    |    7.62° (UP)   |    6.27° (DOWN)
//  10° (LEFT)    |    4.92° (UP)   |    4.27° (DOWN)
//  5° (LEFT)     |    2.36° (UP)   |    2.18° (DOWN)
//  0°            |    0°           |    0°
//  5° (RIGHT)    |    2.24° (DOWN) |    2.29° (UP)
//  10° (RIGHT)   |    4.36° (DOWN) |    4.74° (UP)
//  15° (RIGHT)   |    6.43° (DOWN) |    7.33° (UP)
//  20° (RIGHT)   |    8.33° (DOWN) |    10.15° (UP)
//  25° (RIGHT)   |    10.21° (DOWN)|    13.22° (UP)
//  30° (RIGHT)   |    11.96° (DOWN)|    16.63° (UP)
//  35° (RIGHT)   |    13.67° (DOWN)|    20.51° (UP)
//  40° (RIGHT)   |    15° (DOWN)   |    25° (UP)

// left will be negative in the yoke values, negative is down (duh)
const INTERPOLATION_POINTS: [AileronInterpolation; 17] = [
    AileronInterpolation {
        yoke_input: -40.0,
        left_aileron: 25.0,
        right_aileron: -15.0,
    },
    AileronInterpolation {
        yoke_input: -35.0,
        left_aileron: 20.65,
        right_aileron: -13.54,
    },
    AileronInterpolation {
        yoke_input: -30.0,
        left_aileron: 17.01,
        right_aileron: -11.84,
    },
    AileronInterpolation {
        yoke_input: -25.0,
        left_aileron: 13.59,
        right_aileron: -10.06,
    },
    AileronInterpolation {
        yoke_input: -20.0,
        left_aileron: 10.52,
        right_aileron: -8.20,
    },
    AileronInterpolation {
        yoke_input: -15.0,
        left_aileron: 7.62,
        right_aileron: -6.27,
    },
    AileronInterpolation {
        yoke_input: -10.0,
        left_aileron: 4.92,
        right_aileron: -4.27,
    },
    AileronInterpolation {
        yoke_input: -5.0,
        left_aileron: 2.36,
        right_aileron: -2.18,
    },
    AileronInterpolation {
        yoke_input: 0.0,
        left_aileron: 0.0,
        right_aileron: 0.0,
    },
    AileronInterpolation {
        yoke_input: 5.0,
        left_aileron: 2.24,
        right_aileron: 2.29,
    },
    AileronInterpolation {
        yoke_input: 10.0,
        left_aileron: -4.36,
        right_aileron: 4.74,
    },
    AileronInterpolation {
        yoke_input: 15.0,
        left_aileron: -6.43,
        right_aileron: 7.33,
    },
    AileronInterpolation {
        yoke_input: 20.0,
        left_aileron: -8.33,
        right_aileron: 10.15,
    },
    AileronInterpolation {
        yoke_input: 25.0,
        left_aileron: -10.21,
        right_aileron: 13.22,
    },
    AileronInterpolation {
        yoke_input: 30.0,
        left_aileron: -11.96,
        right_aileron: 16.63,
    },
    AileronInterpolation {
        yoke_input: 35.0,
        left_aileron: -13.67,
        right_aileron: 20.51,
    },
    AileronInterpolation {
        yoke_input: 40.0,
        left_aileron: -15.0,
        right_aileron: 25.0,
    },
];
// (Left, Right)
pub fn get_required_aileron_deflection(yoke_input: f64) -> (f64, f64) {
    let (lower, upper) = INTERPOLATION_POINTS
        .iter()
        .enumerate()
        .find(|(_, &point)| point.yoke_input >= yoke_input)
        .map(|(indx, _)| {
            if indx == 0 {
                (INTERPOLATION_POINTS[0], INTERPOLATION_POINTS[1])
            } else {
                (INTERPOLATION_POINTS[indx - 1], INTERPOLATION_POINTS[indx])
            }
        })
        .unwrap_or((INTERPOLATION_POINTS[15], INTERPOLATION_POINTS[16]));

    let t = (yoke_input - lower.yoke_input) / (upper.yoke_input - lower.yoke_input);
    let left_aileron = lower.left_aileron + t * (upper.left_aileron - lower.left_aileron);
    let right_aileron = lower.right_aileron + t * (upper.right_aileron - lower.right_aileron);

    (left_aileron, right_aileron)
}
