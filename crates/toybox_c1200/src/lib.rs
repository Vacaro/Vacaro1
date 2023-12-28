use include_dir::{include_dir, Dir};
use nih_plug::prelude::*;
use presets::Presets;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
mod presets;

static ASSETS: Dir<'_> = include_dir!("D:/Github/zmann-vst/bin_samples/ToyboxC1200");

struct ToyboxC {
    params: Arc<ToyboxCParams>,
    pub buffer: Vec<instrument::buffer::Sample>,
    instrument: instrument::binv3::Instrument,
}

#[derive(Params)]
struct ToyboxCParams {
    #[id = "output"]
    pub output: FloatParam,
    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    preset_changed: Arc<AtomicBool>,
}

impl Default for ToyboxC {
    fn default() -> Self {
        Self {
            params: Arc::new(ToyboxCParams::default()),
            buffer: vec![],
            instrument: instrument::binv3::Instrument::empty(),
        }
    }
}

impl Default for ToyboxCParams {
    fn default() -> Self {
        let preset_changed = Arc::new(AtomicBool::new(false));
        let preset_changed_mem = preset_changed.clone();
        let preset_callback = Arc::new(move |_: Presets| {
            preset_changed_mem.store(true, Ordering::Relaxed);
        });
        Self {
            output: FloatParam::new(
                "Output Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            preset: EnumParam::new("Preset", Presets::default()).with_callback(preset_callback), // .hide(),
            preset_changed,
        }
    }
}

impl ToyboxC {
    fn load_preset(&mut self, preset: Presets) {
        nih_log!("[Toybox C1200] load_preset: {:?}", preset);
        if let Some(input_file) = ASSETS.get_file(format!("{}.binv3", preset.to_string())) {
            self.instrument = instrument::binv3::decode(input_file.contents().to_vec());
            nih_log!("[Toybox C1200] load_preset done: {:?}", preset);
        }
    }
}

impl Plugin for ToyboxC {
    const NAME: &'static str = "Toybox C1200";
    const VENDOR: &'static str = "ZMANN";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.load_preset(self.params.preset.value());
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let output = self.params.output.smoothed.next();

            for sample in channel_samples {
                *sample *= output;
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for ToyboxC {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.c120012345";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_vst3!(ToyboxC);
