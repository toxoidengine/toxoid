use toxoid_sdl::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, _out: &mut [u8]) {
        // ... [no changes here] ...
    }
}

pub fn init() {
    toxoid_sdl::AUDIO_SUBSYSTEM.with(|audio| {
        let audio_subsystem = audio.borrow();
        
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default
        };
        let wav_file: Cow<'static, Path> = match std::env::args().nth(1) {
            None => Cow::from(Path::new("assets/sound.wav")),
            Some(s) => Cow::from(PathBuf::from(s)),
        };
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load test WAV data");

            let cvt = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
            .expect("Could not convert WAV data");

            let data = cvt.convert(wav.buffer().to_vec());

            // initialize the audio callback
            Sound {
                data,
                volume: 0.01,
                pos: 0,
            }
        }).unwrap();
        
        // Start playback
        device.resume();
        // core::mem::forget(device);
    });
}