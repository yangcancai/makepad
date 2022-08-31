pub use makepad_component;
pub use makepad_component::makepad_platform;
pub use makepad_platform::makepad_math;
pub use makepad_media;

use makepad_component::*;
use makepad_component::imgui::*;
use makepad_draw_2d::*;
use makepad_media::*;
use makepad_media::audio_graph::*;

mod display_audio;
mod piano;
mod iron_fish;

use crate::iron_fish::*;
use crate::piano::*;
use crate::display_audio::*;

live_register!{
    registry AudioComponent::*;
    registry FrameComponent::*;
    import makepad_component::theme::*;
    import makepad_component::frame::*;
    import makepad_draw_2d::shader::std::*;
    
    MainHeader: FoldHeader {
        state: {
            open = {
                off = {apply: {header: {bg: {radius: vec2(3.0, 3.0)}}}}
                on = {apply: {header: {bg: {radius: vec2(3.0, 1.0)}}}}
            }
        }
        header: BoxY {
            cursor: Default,
            bg: {color: #6},
            walk: {width: Fill, height: Fit},
            layout: {flow: Right, padding: 8, spacing: 5}
        }
    }
    
    InstrumentHeader: FoldHeader {
        header: Rect {
            cursor: Default,
            bg: {color: #5},
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, padding: 8, spacing: 5}
        }
    }
    
    LayerHeader: InstrumentHeader {
        header: {
            bg: {color: #4},
        }
    }
    
    FoldablePiano: MainHeader {
        header: {
            fold_button = FoldButton {}
            label = Label {text: "Keys"}
        }
        state: {
            open = {
                off = {apply: {body: {g1 = {bg: {color: #0000}}}}}
                on = {apply: {body: {g1 = {bg: {color: #000a}}}}}
            }
        }
        body: Frame {
            layout: {flow: Overlay}
            walk: {width: Fit, height: Fit}
            Frame {
                layout: {flow: Down}
                walk: {width: Fit, height: Fit},
                piano = Piano {}
                GradientY {
                    walk: {width: Fill, height: 10}
                    bg: {color: #000a, color2: #0000}
                }
            }
            g1 = GradientY {
                walk: {width: Fill, height: 2}
                bg: {color: #000a, color2: #0000}
            }
        }
    }
    
    ElementBox: Frame {
        bg: {color: #4}
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, padding: 8, spacing: 5}
    }
    
    InstrumentSlider: ElementBox {
        slider = Slider {
            label: "CutOff1"
            walk:{height: 22}
        }
    }
    
    InstrumentCheckbox: ElementBox {
        checkbox = CheckBox {
            label: "CutOff1"
        }
    }
    
    TextInputTest: ElementBox {
        layout: {padding: {left: 8}}
        textbox = TextInput {
            text: "Hello WOrld"
        }
    }
    
    ListBoxTest: ElementBox {
        listbox = ListBox {
            items: ["One", "Two", "Three", "Four", "Five", "Six"]
        }
    }
    
    InstrumentDropdown: ElementBox {
        layout: {align:{y:0.5}, padding: 5, flow:Right}
        label = Label{walk:{width:70,margin:{left:4}}}
        dropdown = DropDown {}
    } 
     
    EnvelopePanel:Frame{
        layout: {flow: Right}
        walk: {width: Fill, height: Fit}
        attack = InstrumentSlider {
            slider = {
                bind: "adsr.a"
                min: 0.0
                max: 1.0
                label: "A"
            }
        }
        hold = InstrumentSlider {
            slider = {
                bind: "adsr.h"
                min: 0.0
                max: 1.0
                label: "H"
            }
        }
        decay = InstrumentSlider {
            slider = {
                bind: "adsr.d"
                min: 0.0
                max: 1.0
                label: "D"
            }
        }
        sustain = InstrumentSlider {
            slider = {
                bind: "adsr.s"
                min: 0.0
                max: 1.0
                label: "S"
            }
        }
        release = InstrumentSlider {
            slider = {
                bind: "adsr.r"
                min: 0.0
                max: 1.0
                label: "R"
            }
        }
    }
    
    FishHeader: Frame{     
        walk:{width: Fill, height: Fit}
        layout:{padding: 5}
        label = Label{
            label:{text_style:{font_size: 12}, color: #f}
            text: "replace me!"
        }
    }

    FishPanel:Box{
        layout: {flow: Down}
        walk: {width: Fill, height: Fit}
        label = FishHeader{label={text:"ReplaceMe"}}    
        bg: {
            color: #804030, 
            color2: #404040, 
            fn get_fill(self)->vec4{
                return mix(self.color, self.color2, clamp((self.pos.y * self.rect_size.y)/100,0,1))
            }
        }
    }

    TouchPanel: FishPanel{
        bg: {color: #f08000}

        label = {label={text:"Touch"}}
    }

    MixerPanel: FishPanel{
        bg: {color: #d0d0d0}
        label = {label={text:"Mixer"}}

        noise = InstrumentSlider {
            slider = {
                bind: "noise"
                min: 0.0
                max: 1.0
                label: "Noise"
            }
        }
        sub = InstrumentSlider {
            slider = {
                bind: "sub_osc"
                min: 0.0
                max: 1.0
                label: "Sub Oscillator"
            }
        }

    }
    FXPanel: FishPanel{
        bg: {color: #8080f0}
        label = {label={text:"FX"}}
    }
    VolumeEnvelopePanel:FishPanel{
        bg: {color: #f08000}
        label = {label={text:"Volume Envelope"}}
        env = EnvelopePanel{
            attack={slider={bind:"volume_envelope.a"}}
            hold={slider={bind:"volume_envelope.h"}}
            decay={slider={bind:"volume_envelope.d"}}
            sustain={slider={bind:"volume_envelope.s"}}
            release={slider={bind:"volume_envelope.r"}}
        }
    }

    ModEnvelopePanel:FishPanel{

        bg: {color: #f08000}
        label = {label={text:"Modulation Envelope"}}
        env = EnvelopePanel{
            attack={slider={bind:"mod_envelope.a"}}
            hold={slider={bind:"mod_envelope.h"}}
            decay={slider={bind:"mod_envelope.d"}}
            sustain={slider={bind:"mod_envelope.s"}}
            release={slider={bind:"mod_envelope.r"}}
        }
    }

    FilterPanel:FishPanel{

        bg: {color: #0000f0}

        label = {label={text:"Filter"}}
        InstrumentDropdown {
            label = {text: "Filter"}
            dropdown = {
                bind_enum: "FilterType"
                bind: "filter1.filter_type"
                items: ["Lowpass", "Highpass", "Bandpass"]
            }
        }

        cutoff = InstrumentSlider {
            slider = {
                bind: "filter1.cutoff"
                min: 0.0
                max: 1.0
                label: "Cutoff"
            }
        }
        resonance = InstrumentSlider {
            slider = {
                bind: "filter1.resonance"
                min: 0.0
                max: 1.0
                label: "Resonance"
            }
        }
        
        modamount = InstrumentSlider {
            slider = {
                bind: "filter1.envelope_amount"
                min: -1.0
                max: 1.0
                label: "Mod Env Amount"
            }
        }
        
        touchamount = InstrumentSlider {
            slider = {
                bind: "filter1.touch_amount"
                min: -1.0
                max: 1.0
                label: "Touch Amount"
            }
        }
    }

    OscPanel:FishPanel{
        label = {label={text:"Oscillator ?"}}
        bg: {color: #f0f000}
        type = InstrumentDropdown {
            label = {text: "Osc1 type"}
            dropdown = {
                bind_enum: "OscType"
                bind: "osc1.osc_type"
                items: ["DPWSawPulse", "TrivialSaw", "BlampTri", "Naive", "Pure"]
            }
        }
        transpose = InstrumentSlider {
            slider = {
                bind: "osc1.transpose"
                min: 0.0
                max: 36.0
                label: "Transpose"
            }
        }
        detune = InstrumentSlider {
            slider = {
                bind: "osc1.detune"
                min: 0.0
                max: 1.0
                label: "Detune"
            }
        }
    }
    
    IronFishUI: InstrumentHeader {
        header: {
            layout: {align: {y: 0.5}}
            fold_button = FoldButton {}
            swatch = Circle {
                walk: {width: 10, height: 10}
                bg: {color: #f00}
            }
            label = Label {text: "IronFish"}
        }
        body: Frame {
            layout: {flow: Down}
            walk: {width: Fill, height: Fit}
            stack = LayerHeader {
                walk: {width: Fill, height: Fit}
                header: {
                    fold_button = FoldButton {}
                    label = Label {text: "Stack item", walk: {width: Fill}}
                }
                body: Frame {
                    layout: {flow: Down}
                    walk: {width: Fill, height: Fit}
                    
                    
                    

                    /*
                    InstrumentSlider {
                        slider = {
                            bind: "filter1.cutoff"
                            min: 0.0
                            max: 1.0
                            label: "Filter Cutoff"
                        }
                    }
                    InstrumentSlider {
                        slider = {
                            bind: "filter1.resonance"
                            min: 0.02
                            max: 1.0
                            label: "Filter Resonance"
                        }
                    }
                    InstrumentSlider {
                        slider = {
                            bind: "filter1.envelope_amount"
                            min: 0.0
                            max: 1.0
                            label: "Envelope Amount"
                        }
                    }
                    InstrumentSlider {
                        slider = {
                            bind: "osc1.detune"
                            min: 0.0
                            max: 10.0
                            label: "Osc1 detune"
                        }
                    }
                    InstrumentSlider {
                        slider = {
                            bind: "osc2.detune"
                            min: 0.0
                            max: 10.0
                            label: "Osc2 detune"
                        }
                    }
                    InstrumentDropdown {
                        label = {text: "Osc1 type"}
                        dropdown = {
                            bind_enum: "OscType"
                            bind: "osc1.osc_type"
                            items: ["DPWSawPulse", "TrivialSaw", "BlampTri", "Naive", "Pure"]
                        }
                    }
                    InstrumentDropdown {
                        label = {text: "Osc2 type"}
                        dropdown = {
                            bind_enum: "OscType"
                            bind: "osc2.osc_type"
                            items: ["DPWSawPulse", "TrivialSaw", "BlampTri", "Naive", "Pure"]
                        }
                    }
                    InstrumentDropdown {
                        label = {text: "Filter"}
                        dropdown = {
                            bind_enum: "FilterType"
                            bind: "filter1.filter_type"
                            items: ["Lowpass", "Highpass", "Bandpass"]
                        }
                    }
                    InstrumentCheckbox{
                        checkbox = {
                            label:"Hello world"
                        }
                    }*/
                }
            }
        }
    }
    
    App: {{App}} {
        window: {window:{inner_size: vec2(1280,1000)},pass: {clear_color: (COLOR_BG_APP)}}
        audio_graph: {
            root: Mixer {
                c1 = Instrument {
                    
                    
                  /*  AudioUnitInstrument {
                        plugin: "Kontakt"
                    }*/
                    
                    IronFish {
                    }
                    
                    //key_range: {start: 34, end: 47 shift: 30}
                    /*
                    AudioUnitInstrument {
                        plugin: "Kontakt"
                    }*/
                }
            }
        }
        imgui: {
            design_mode: false,
            bg: {color: (COLOR_BG_APP)},
            walk: {width: Fill, height: Fill}
            layout: {
                padding: 8
                align: {x: 0.0, y: 0.0}
                spacing: 5.,
                flow: Flow::Down
            },
            Frame {
                layout: {flow: Right, spacing: 5.0}
                walk: {margin: {left: 60}, height: Fit}
                Button {text: "+  Band"}
                Button {text: "<"}
                Button {text: ">"}
                Solid {
                    walk: {width: Fill, height: 36}
                    bg: {
                        const WAVE_HEIGHT: 0.15
                        const WAVE_FREQ: 0.2
                        fn pixel(self) -> vec4 {
                            let offset_y = 1.5
                            let pos2 = vec2(self.pos.x, self.pos.y + WAVE_HEIGHT * sin(WAVE_FREQ * self.pos.x * self.rect_size.x))
                            let sdf = Sdf2d::viewport(pos2 * self.rect_size)
                            sdf.clear(#2f)
                            sdf.move_to(0., self.rect_size.y * 0.5)
                            sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5)
                            return sdf.stroke(#f, 1.0)
                        }
                    }
                }
            }
                     
            FoldablePiano {}
            OscPanel{
                label={label={text:"Oscillator 1"}}                    
                type={dropdown={bind:"osc1.osc_type"}}
                transpose={slider={bind:"osc1.transpose"}}
                detune={slider={bind:"osc1.detune"}}
            }
            MixerPanel{}
            OscPanel{
                label={label={text:"Oscillator 2"}}                    
                type={dropdown={bind:"osc2.osc_type"}}
                transpose={slider={bind:"osc2.transpose"}}
                detune={slider={bind:"osc2.detune"}}
            }
            FilterPanel{}
            VolumeEnvelopePanel{}
            ModEnvelopePanel{}
            TouchPanel{}
           
            FXPanel{}
            display_audio = DisplayAudio {
                walk: {height: Fill, width: Fill}
            }                    
        }
    }
}
main_app!(App);

#[derive(Live, LiveHook)]
pub struct App {
    imgui: ImGUI,
    audio_graph: AudioGraph,
    window: BareWindow,
    data: f32,
}

impl App {
    pub fn live_register(cx: &mut Cx) {
        makepad_component::live_register(cx);
        makepad_media::live_register(cx);
        crate::display_audio::live_register(cx);
        crate::iron_fish::live_register(cx);
        crate::piano::live_register(cx);
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return Cx2d::draw(cx, event, self, | cx, s | s.draw(cx));
        }
        //let dt = profile_start();
        
        self.window.handle_event(cx, event);
        
        let mut ui = self.imgui.run(cx, event);
        
        if ui.on_construct() {
            ui.cx.start_midi_input();
            let iron_fish = self.audio_graph.by_type::<IronFish>().unwrap();
            ui.bind_read(&iron_fish.settings.live_read());
            ui.piano(ids!(piano)).set_key_focus(ui.cx);
        }
        
        let display_audio = ui.display_audio(ids!(display_audio));
        
        let mut buffers = 0;
        self.audio_graph.handle_event(ui.cx, ui.event, &mut | cx, action | {
            match action {
                AudioGraphAction::DisplayAudio {buffer, voice, active} => {
                    display_audio.process_buffer(cx, active, voice, buffer);
                    buffers += 1;
                }
                AudioGraphAction::VoiceOff {voice} => {
                    display_audio.voice_off(cx, voice);
                }
            }
        });
        
        // fetch ui binding deltas
        
        for delta in ui.on_bind_deltas() {
            log!("{}", delta.to_string(0,100));
            let iron_fish = self.audio_graph.by_type::<IronFish>().unwrap();
            iron_fish.settings.apply_over(ui.cx, &delta);
            ui.bind_read(&delta);
        }
        
        let piano = ui.piano(ids!(piano));
        
        for inp in ui.cx.on_midi_1_input_data(ui.event) {
            self.audio_graph.send_midi_1_data(inp.data);
            if let Some(note) = inp.data.decode().on_note() {
                piano.set_note(ui.cx, note.is_on, note.note_number)
            }
        }
        
        for note in piano.on_notes() {
            self.audio_graph.send_midi_1_data(Midi1Note {
                channel: 0,
                is_on: note.is_on,
                note_number: note.note_number,
                velocity: note.velocity
            }.into());
        }
        //profile_end(dt);
    }
    
    pub fn draw(&mut self, cx: &mut Cx2d) {
        if self.window.begin(cx).not_redrawing() {
            return;
        }
        
        while self.imgui.draw(cx).is_not_done() {};
        
        self.window.end(cx);
    }
}