use wgpu::*;

#[async_std::main]
async fn main() {
    let instance = Instance::new();
    let adapter = instance
        .request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::Default,
                compatible_surface: None,
            },
            BackendBit::PRIMARY,
        )
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                limits: Limits::default(),
                extensions: Extensions::default(),
            },
            None,
        )
        .await
        .unwrap();

    let parent_buffer = device.create_buffer(&BufferDescriptor {
        size: 100_000,
        usage: BufferUsage::STORAGE | BufferUsage::COPY_SRC,
        label: None,
    });

    loop {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        let child_buffer = device.create_buffer(&BufferDescriptor {
            size: 100_000,
            usage: BufferUsage::MAP_READ | BufferUsage::COPY_DST,
            label: None,
        });
        encoder.copy_buffer_to_buffer(&parent_buffer, 0, &child_buffer, 0, 100_000);
        queue.submit(std::iter::once(encoder.finish()));
        let fut = child_buffer.map_read(0, 100_000);
        device.poll(Maintain::Wait);
        fut.await.unwrap();
    }
}
