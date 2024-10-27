/// Quadcopter routines

struct CopterSet {
}

/// Composite measured state (sensor readings)
pub struct SensorState {
}

/// Complete system state.
/// - TODO: move into separate module on maturity
pub struct SystemState {
}

pub type QuadControl = [f32; 4];

/// - TODO: geometry
pub struct Quad {
    /// Torque proportionality constant reqceived from BLDC motors (1). Does not
    /// take blade properties into account.
    motor_torque_coeff: f32,
}

impl Quad {
    /// Sets the engines' relative throttle. `control` compises 3 values scaled
    /// from 0 to 1
    pub fn apply_control(&mut self, control: &QuadControl) {
        // TODO: rigid body
    }

    /// System state in the worlds' coordinates: position, rotation, velocity,
    /// acceleration, torque
    pub fn system_state(&self) -> SystemState {
        // TODO
        SystemState {}
    }

    pub fn sensor_state(&self) -> SensorState {
        SensorState {}
    }
}

pub const DEMO_QUAD: Quad = Quad {
    motor_torque_coeff: 0.12f32,
};
