use nih_plug::prelude::*;
use nih_plug_webview::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ts_rs::TS;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

struct Gain {
    params: Arc<GainParams>,
    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,
    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,
}

// "Run Test" (at least, in vscode) will (re-) generate the TS bindings
#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
#[ts(export_to = "../gui/bindings/Action.ts")]
#[serde(tag = "type")]
enum Action {
    SetGain { value: f32 },
}
#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
#[ts(export_to = "../gui/bindings/PluginMessage.ts")]
#[serde(tag = "type")]
enum PluginMessage {
    ParamChange { param: String, value: f32 },
    PeakMeterData { value: f32 },
}

#[derive(Params)]
struct GainParams {
    #[id = "gain"]
    pub gain: FloatParam,
    gain_value_changed: Arc<AtomicBool>,
}

impl Default for Gain {
    fn default() -> Self {
        Self {
            params: Arc::new(GainParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Default for GainParams {
    fn default() -> Self {
        let gain_value_changed = Arc::new(AtomicBool::new(false));

        let v = gain_value_changed.clone();
        let param_callback = Arc::new(move |val: f32| {
            nih_log!("Value changed: {}", val);
            v.store(true, Ordering::Relaxed);
        });

        Self {
            gain: FloatParam::new(
                "Gain",
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
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_callback(param_callback.clone()),
            gain_value_changed,
        }
    }
}

impl Plugin for Gain {
    type BackgroundTask = ();
    type SysExMessage = ();

    const NAME: &'static str = "Next-Gain";
    const VENDOR: &'static str = "DVUB";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = "0.0.1";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
                amplitude += *sample;
            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open

            amplitude = (amplitude / num_samples as f32).abs();
            let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
            let new_peak_meter = if amplitude > current_peak_meter {
                amplitude
            } else {
                current_peak_meter * self.peak_meter_decay_weight
                    + amplitude * (1.0 - self.peak_meter_decay_weight)
            };
            // nih_log!("{}", new_peak_meter);

            self.peak_meter
                .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
        }

        ProcessStatus::Normal
    }
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let gain_value_changed = self.params.gain_value_changed.clone();
        let peak_meter = self.peak_meter.clone();

        let size = (300, 450);
        #[cfg(debug_assertions)]
        let src = HTMLSource::URL("http://localhost:3000".to_owned());
        let mut editor = WebViewEditor::new(src, size);
        #[cfg(not(debug_assertions))]
        let mut editor =
            editor_with_frontend_dir("D:\\projects\\rust\\next-gain\\gui\\out".into(), size, None);

        editor = editor
            .with_developer_mode(true)
            .with_keyboard_handler(move |event| {
                println!("keyboard event: {event:#?}");
                event.key == Key::Escape
            })
            .with_mouse_handler(|event| match event {
                MouseEvent::DragEntered { .. } => {
                    println!("drag entered");
                    EventStatus::AcceptDrop(DropEffect::Copy)
                }
                MouseEvent::DragMoved { .. } => {
                    println!("drag moved");
                    EventStatus::AcceptDrop(DropEffect::Copy)
                }
                MouseEvent::DragLeft => {
                    println!("drag left");
                    EventStatus::Ignored
                }
                MouseEvent::DragDropped { data, .. } => {
                    if let DropData::Files(files) = data {
                        println!("drag dropped: {:?}", files);
                    }
                    EventStatus::AcceptDrop(DropEffect::Copy)
                }
                _ => EventStatus::Ignored,
            })
            .with_event_loop(move |ctx, setter, _window| {
                let mut sent_from_gui = false;
                while let Ok(value) = ctx.next_event() {
                    if let Ok(action) = serde_json::from_value(value) {
                        #[allow(clippy::single_match)]
                        match action {
                            Action::SetGain { value } => {
                                sent_from_gui = true;
                                setter.begin_set_parameter(&params.gain);
                                setter.set_parameter(&params.gain, value);
                                setter.end_set_parameter(&params.gain);
                            } // I was having trouble getting resizing to work so i just decided to scrap it
                              // i am a bad programmer :(
                              /*
                              Action::SetSize { width, height } => {
                                  ctx.resize(window, width, height);
                              }

                              Action::Init => {
                                  let _ = ctx.send_json(json!({
                                      "type": "set_size",
                                      "width": ctx.width.load(Ordering::Relaxed),
                                      "height": ctx.height.load(Ordering::Relaxed)
                                  }));*/
                        }
                    } else {
                        panic!("Invalid action received from web UI.")
                    }
                }

                if !sent_from_gui && gain_value_changed.swap(false, Ordering::Relaxed) {
                    let data = PluginMessage::ParamChange {
                        param: "gain".to_owned(),
                        value: params.gain.value(),
                    };
                    let _ = ctx.send_json(json!(data));
                }
                let v = peak_meter.load(Ordering::Relaxed);
                // println!("{}", v);
                let data = PluginMessage::PeakMeterData { value: v };
                let _ = ctx.send_json(json!(data));
            });

        Some(Box::new(editor))
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Gain {
    const VST3_CLASS_ID: [u8; 16] = *b"GainMoistestPlug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(Gain);
