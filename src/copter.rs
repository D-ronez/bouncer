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

pub struct MassProperties {
    /// Moment of inertia in body space, 3-vector, NEU
    pub ibody: [f32; 3],

    /// Mass, [g]
    pub mass: f32,

    /// Center of mass position in body space, 3-vector, [m]
    pub com: [f32; 3],
}

/// Describes quadcopter's properties.
pub struct Quad {
    /// Torque proportionality constant received from BLDC motors (1). Does not
    /// take blade properties into account.
    pub motor_torque_coeff: f32,

    pub mass_properties: MassProperties,

    /// Dimensions of the drone, 3-vector, NEU, [m]
    pub size: [f32; 3],
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

/// A small lightweight drone
pub const DEMO_QUAD: Quad = Quad {
    motor_torque_coeff: 0.12f32,
    mass_properties: MassProperties {
        ibody: [0.0087f32, 0.0133f32, 0.0087f32],
        mass: 500f32,
        com: [0.0f32, 0.0f32, 0.0f32],
    },
    size: [0.03f32, 0.03f32, 0.08f32],
};
