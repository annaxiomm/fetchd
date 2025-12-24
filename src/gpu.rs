use wgpu::Instance;

pub fn get_gpu_name() -> Option<String>{
    let instance = Instance::default();

    let adapter = pollster::block_on(
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }),
    )
    .expect("No GPU found");

    Some(adapter.get_info().name)
}