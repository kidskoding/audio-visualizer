use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use color_eyre::eyre::Result;

pub fn capture_audio() -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");

    let mut config_range = device
        .supported_input_configs()
        .expect("error while querying input configs");
    let config = config_range
        .next()
        .expect("no supported input config?!")
        .with_max_sample_rate();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // 👇 Handle incoming audio data here (visualization, FFT, etc)
            println!("Received {} samples", data.len());
        },
        move |err| {
            eprintln!("Stream error: {}", err);
        },
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
