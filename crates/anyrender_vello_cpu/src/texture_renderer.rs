use std::sync::{Arc, Mutex};

use crate::{ImageCacheConfig, VelloCpuScenePainter};
use anyrender::{ImageRenderer, RenderContext as AnyRenderContext, TextureRenderer};
use debug_timer::debug_timer;
use vello_cpu::{Pixmap, PixmapMut, RenderContext};

pub struct VelloCpuTexture(Arc<Mutex<Pixmap>>);

pub struct VelloCpuTextureRenderer {
    scene: VelloCpuScenePainter,
}

impl VelloCpuTextureRenderer {
    /// Create a renderer with a custom image cache configuration.
    pub fn with_image_cache_config(width: u32, height: u32, config: ImageCacheConfig) -> Self {
        Self {
            scene: VelloCpuScenePainter::with_image_cache_config(
                width as u16,
                height as u16,
                config,
            ),
        }
    }

    /// Drop all cached image conversions.
    pub fn clear_image_cache(&mut self) {
        self.scene.clear_image_cache();
    }
}

impl AnyRenderContext for VelloCpuTextureRenderer {}
impl TextureRenderer for VelloCpuTextureRenderer {
    type ScenePainter<'a> = VelloCpuScenePainter;

    fn render_to_texture<F: FnOnce(&mut Self::ScenePainter<'_>)>(
        &mut self,
        draw_fn: F,
        texture: &dyn anyrender::Texture,
    ) {
        debug_timer!(timer, feature = "log_frame_times");

        draw_fn(&mut self.scene);
        timer.record_time("cmds");

        self.scene.render_ctx.flush();
        timer.record_time("flush");

        self.scene.render_ctx.render(
            ,
            &mut self.scene.resources,
        );
        timer.record_time("render");

        self.scene.maintain();
        timer.record_time("maintain");

        timer.print_times("vello_cpu: ");
    }
}
