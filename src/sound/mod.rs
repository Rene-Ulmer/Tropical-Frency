use alloc::vec::Vec;

mod tracks;

extern "C" {
    fn P(freq: f32) -> usize;
    fn S(handle: usize);
}

fn play_tone(freq: f32) -> usize {
    unsafe { P(freq) }
}

fn stop_tone(handle: usize) {
    unsafe {
        S(handle);
    }
}

const TWELTH_ROOT_OF_TWO: f32 = 1.059463094359f32;

// Relative to the 'C', n in half-steps.
fn frequency_for_note(n: isize) -> f32 {
    let times = (if n > 0 { n } else { -n }) as usize;
    let mut factor = 1f32;
    for _ in 0..times {
        factor *= TWELTH_ROOT_OF_TWO;
    }

    if n > 0 {
        440f32 * factor
    } else {
        440f32 / factor
    }
}

#[repr(packed)]
pub struct Note {
    note: i8,
    begin: u16,
    duration: u8,
}

pub struct MusicTrack {
    notes: &'static [Note],
    notes_to_disable: Vec<(usize, f32)>,
    current_time: f32,
    multiplier: usize,
}

impl MusicTrack {
    pub fn new(notes: &'static [Note], multiplier: usize) -> Self {
        Self {
            notes,
            notes_to_disable: Vec::new(),
            current_time: 0f32,
            multiplier,
        }
    }
    pub fn update(&mut self, delta: f32, base: i8) {
        // Check whether to loop back
        if self.current_time > 2128f32 * self.multiplier as f32 {
            self.current_time = 271.9f32 * self.multiplier as f32
        }
        let old_time = self.current_time;
        self.current_time += delta * 1.65f32; // / 1000f32 * 480f32;

        // Check whether we need to play a new sound.
        for note in self.notes {
            if (note.begin as f32 * self.multiplier as f32) > old_time
                && (note.begin as f32 * self.multiplier as f32) < self.current_time
            {
                let idx = play_tone(frequency_for_note(note.note as isize - base as isize));
                self.notes_to_disable.push((
                    idx,
                    (note.begin as f32 + note.duration as f32) * self.multiplier as f32,
                ));
            }
        }

        // Check whether we need to stop sounds.
        let mut i = 0;
        while i < self.notes_to_disable.len() {
            if (self.notes_to_disable[i].1 < self.current_time)
                || (self.notes_to_disable[i].1 > 2128f32 * self.multiplier as f32)
            {
                stop_tone(self.notes_to_disable[i].0);
                self.notes_to_disable.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

pub struct Music {
    tracks: Vec<MusicTrack>,
}

impl Music {
    pub fn new() -> Self {
        let mut tracks = Vec::new();
        tracks.push(MusicTrack::new(self::tracks::PTRACK1, 120));
        tracks.push(MusicTrack::new(self::tracks::PTRACK2, 120));

        Self { tracks }
    }

    pub fn update(&mut self, delta: f32) {
        //for track in self.tracks.iter_mut() {
        //    track.update(delta);
        self.tracks[0].update(delta, 61);
        self.tracks[1].update(delta, 49);
        //}
    }
}
