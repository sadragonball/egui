mod volume_renderer;

pub struct VolumeRenderer{

}

impl VolumeRenderer{
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().expect("wgpu enabled");
        let device = &wgpu_render_state.device;
    }
}