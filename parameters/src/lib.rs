extern crate lv2_sys as sys;

use urid::*;

pub mod parameters {
    use urid::UriBound;

    pub struct CompressorControlsClass;
    unsafe impl UriBound for CompressorControlsClass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__CompressorControls;
    }

    pub struct ControlGroupClass;
    unsafe impl UriBound for ControlGroupClass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__ControlGroup;
    }

    pub struct EnvelopeControlsClass;
    unsafe impl UriBound for EnvelopeControlsClass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__EnvelopeControls;
    }

    pub struct FilterControlsClass;
    unsafe impl UriBound for FilterControlsClass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__FilterControls;
    }

    pub struct OscillatorControlsClass;
    unsafe impl UriBound for OscillatorControlsClass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__OscillatorControls;
    }

    pub struct Amplitude;
    unsafe impl UriBound for Amplitude {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__amplitude;
    }

    pub struct Attack;
    unsafe impl UriBound for Attack {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__attack;
    }

    pub struct Bypass;
    unsafe impl UriBound for Bypass {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__bypass;
    }

    pub struct CutoffFrequency;
    unsafe impl UriBound for CutoffFrequency {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__cutoffFrequency;
    }

    pub struct Decay;
    unsafe impl UriBound for Decay {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__decay;
    }

    pub struct Delay;
    unsafe impl UriBound for Delay {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__delay;
    }

    pub struct DryLevel;
    unsafe impl UriBound for DryLevel {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__dryLevel;
    }

    pub struct Frequency;
    unsafe impl UriBound for Frequency {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__frequency;
    }

    pub struct Gain;
    unsafe impl UriBound for Gain {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__gain;
    }

    pub struct Hold;
    unsafe impl UriBound for Hold {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__hold;
    }

    pub struct PulseWidth;
    unsafe impl UriBound for PulseWidth {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__pulseWidth;
    }

    pub struct Ratio;
    unsafe impl UriBound for Ratio {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__ratio;
    }

    pub struct Release;
    unsafe impl UriBound for Release {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__release;
    }

    pub struct Resonance;
    unsafe impl UriBound for Resonance {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__resonance;
    }

    pub struct SampleRate;
    unsafe impl UriBound for SampleRate {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__sampleRate;
    }

    pub struct Sustain;
    unsafe impl UriBound for Sustain {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__sustain;
    }

    pub struct Threshold;
    unsafe impl UriBound for Threshold {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__threshold;
    }

    pub struct Waveform;
    unsafe impl UriBound for Waveform {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__waveform;
    }

    pub struct WetDryRatio;
    unsafe impl UriBound for WetDryRatio {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__wetDryRatio;
    }

    pub struct WetLevel;
    unsafe impl UriBound for WetLevel {
        const URI: &'static [u8] = sys::LV2_PARAMETERS__wetLevel;
    }
}

#[derive(URIDCollection)]
pub struct ParametersURIDCollection {
    pub compressor_controls_class: URID<parameters::CompressorControlsClass>,
    pub control_group_class: URID<parameters::ControlGroupClass>,
    pub envelope_controls_class: URID<parameters::EnvelopeControlsClass>,
    pub filter_controls_class: URID<parameters::FilterControlsClass>,
    pub oscillator_controls_class: URID<parameters::OscillatorControlsClass>,
    pub amplitude: URID<parameters::Amplitude>,
    pub bypass: URID<parameters::Bypass>,
    pub cutoff_frequency: URID<parameters::CutoffFrequency>,
    pub decay: URID<parameters::Decay>,
    pub delay: URID<parameters::Delay>,
    pub dry_level: URID<parameters::DryLevel>,
    pub frequency: URID<parameters::Frequency>,
    pub gain: URID<parameters::Gain>,
    pub hold: URID<parameters::Hold>,
    pub pulse_width: URID<parameters::PulseWidth>,
    pub ratio: URID<parameters::Ratio>,
    pub release: URID<parameters::Release>,
    pub resonance: URID<parameters::Resonance>,
    pub sample_rate: URID<parameters::SampleRate>,
    pub sustain: URID<parameters::Sustain>,
    pub threshold: URID<parameters::Threshold>,
    pub waveform: URID<parameters::Waveform>,
    pub wet_dry_ratio: URID<parameters::WetDryRatio>,
    pub wet_level: URID<parameters::WetLevel>,
}

pub mod prelude {
    pub use crate::ParametersURIDCollection;
}
