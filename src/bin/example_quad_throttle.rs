use bouncer::copter::{SensorState, SystemState, DEMO_QUAD, QuadControl};
use bouncer::simulation::{Rapier};

fn quad_throttle_control(system_time_us: u64, sensor_state: &SensorState, system_state: &SystemState) -> QuadControl {
    // TODO: generate the actual time-dependent throttle
    [0.0f32, 0.0f32, 0.0f32, 0.0f32]
}

fn main() -> () {
    let mut copter = DEMO_QUAD;
    let mut simulation = Rapier::new();
    simulation.add_quad(&copter);
    loop {
        simulation.step();
        let time = simulation.time_us();
        let control = quad_throttle_control(time, &copter.sensor_state(), &copter.system_state());
        copter.apply_control(&control);
        // TODO: visualization, time scaling
    }
}
