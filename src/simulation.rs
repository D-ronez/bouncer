use crate::copter::Quad;
use nalgebra::Vector3;
use rapier3d::{parry::mass_properties, prelude::*};

pub struct Rapier {
    gravity: Vector3<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
}

impl Rapier {
    pub fn new() -> Rapier {
        Rapier {
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }

    /// Place a physical object into simulation testbed
    /// TODO: consider using collections, like the ones used in Rapier (bodies, colliders, etc.)
    /// TODO: expand to a broader set of objects
    pub fn add_quad(&mut self, quad: &Quad) {
        // Build quad: geometry, and physics
        let mut quad_body = RigidBodyBuilder::dynamic().build();
        quad_body.set_additional_mass_properties(
            MassProperties::new(
                quad.mass_properties.com.into(),
                quad.mass_properties.mass,
                quad.mass_properties.ibody.into(),
            ),
            true,
        );
        let mut quad_body_handle = self.rigid_body_set.insert(quad_body);
        let mut quad_collider_box = ColliderBuilder::new(SharedShape::cuboid(
            quad.size[0],
            quad.size[1],
            quad.size[2],
        ));
        let mut quad_collider_handle = self.collider_set.insert(quad_collider_box);
    }

    pub fn time_us(&self) -> u64 {
        // TODO: return the actual time
        0u64
    }

    pub fn step(&mut self) {}
}
