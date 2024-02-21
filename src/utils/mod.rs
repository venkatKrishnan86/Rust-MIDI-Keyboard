pub fn midi_to_hz(midi: u8) -> Result<f32, String> {
    if midi>=128 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok(f32::powf(2.0, ((midi as f32 - 69 as f32))/12.0) * 440.0)
}

pub fn midi_cents_to_hz(midi: u8, cents_dev: i8) -> Result<f32, String> {
    if cents_dev < -50 && cents_dev > 50 {
        return Err("Cents deviation must be between -50 and 50".to_owned());
    }
    if midi>=128 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok(f32::powf(2.0, ((midi-69) as f32 + cents_dev as f32/100.0)/12.0) * 440.0)
}

// fn hz_to_midi(hz: ) -> f32 {
//     f32::powf(2.0, (midi-69) as f32/12.0) * 440.0
// }

pub fn is_close_f32(a: f32, b: f32) -> bool {
    (a - b).abs() <= 0.01
}