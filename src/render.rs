use crate::simulation::Rapier;
use rapier3d::pipeline::DebugRenderPipeline;
use rapier3d::prelude::*;

/// Can be used ONLY with Rapier
pub struct RapierDebugRender<'a> {
    debug_render_pipeline: DebugRenderPipeline,
    rapier: &'a mut Rapier,
}

impl<'a> RapierDebugRender<'_> {
    pub fn from_rapier(rapier: &'a mut Rapier) -> RapierDebugRender {
        RapierDebugRender {
            debug_render_pipeline: DebugRenderPipeline::new(
                DebugRenderStyle::default(),
                DebugRenderMode::default(),
            ),
            rapier,
        }
    }

    pub fn render(&mut self) {
	}
}
