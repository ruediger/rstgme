use macroquad::audio::{Sound, load_sound_from_bytes, play_sound_once};

const SAMPLE_RATE: u32 = 44100;

/// Audio manager that gracefully handles missing audio support.
/// All sounds are optional - if audio init fails, game continues silently.
pub struct AudioManager {
    enabled: bool,
    // Combat
    knife_swing: Option<Sound>,
    pistol_shot: Option<Sound>,
    shotgun_blast: Option<Sound>,
    machine_pistol: Option<Sound>,
    rifle_shot: Option<Sound>,
    hit: Option<Sound>,
    player_hit: Option<Sound>,
    player_death: Option<Sound>,
    // Pickups
    pickup: Option<Sound>,
    health: Option<Sound>,
    powerup: Option<Sound>,
    // Hacking
    hack_start: Option<Sound>,
    hack_blip: Option<Sound>,
    hack_success: Option<Sound>,
    hack_fail: Option<Sound>,
    game_win: Option<Sound>,
}

async fn try_load_sound(data: &[u8]) -> Option<Sound> {
    load_sound_from_bytes(data).await.ok()
}

impl AudioManager {
    pub async fn load() -> Self {
        // Try to load the first sound to test if audio works
        let test_sound = try_load_sound(&generate_pistol_shot()).await;
        let enabled = test_sound.is_some();

        if !enabled {
            eprintln!("Audio initialization failed - running without sound");
            return Self {
                enabled: false,
                knife_swing: None,
                pistol_shot: None,
                shotgun_blast: None,
                machine_pistol: None,
                rifle_shot: None,
                hit: None,
                player_hit: None,
                player_death: None,
                pickup: None,
                health: None,
                powerup: None,
                hack_start: None,
                hack_blip: None,
                hack_success: None,
                hack_fail: None,
                game_win: None,
            };
        }

        Self {
            enabled: true,
            // Combat sounds
            knife_swing: try_load_sound(&generate_knife_swing()).await,
            pistol_shot: test_sound, // Reuse the test sound
            shotgun_blast: try_load_sound(&generate_shotgun_blast()).await,
            machine_pistol: try_load_sound(&generate_machine_pistol()).await,
            rifle_shot: try_load_sound(&generate_rifle_shot()).await,
            hit: try_load_sound(&generate_hit()).await,
            player_hit: try_load_sound(&generate_player_hit()).await,
            player_death: try_load_sound(&generate_player_death()).await,
            // Pickup sounds
            pickup: try_load_sound(&generate_pickup()).await,
            health: try_load_sound(&generate_health()).await,
            powerup: try_load_sound(&generate_powerup()).await,
            // Hacking sounds
            hack_start: try_load_sound(&generate_hack_start()).await,
            hack_blip: try_load_sound(&generate_hack_blip()).await,
            hack_success: try_load_sound(&generate_hack_success()).await,
            hack_fail: try_load_sound(&generate_hack_fail()).await,
            game_win: try_load_sound(&generate_game_win()).await,
        }
    }

    fn play(&self, sound: &Option<Sound>) {
        if let Some(s) = sound {
            play_sound_once(s);
        }
    }

    pub fn play_shoot(&self, weapon_index: usize) {
        if !self.enabled {
            return;
        }
        let sound = match weapon_index {
            0 => &self.knife_swing,
            1 => &self.pistol_shot,
            2 => &self.shotgun_blast,
            3 => &self.machine_pistol,
            4 => &self.rifle_shot,
            _ => &self.pistol_shot,
        };
        self.play(sound);
    }

    pub fn play_hit(&self) {
        self.play(&self.hit);
    }

    pub fn play_player_hit(&self) {
        self.play(&self.player_hit);
    }

    pub fn play_player_death(&self) {
        self.play(&self.player_death);
    }

    pub fn play_pickup(&self) {
        self.play(&self.pickup);
    }

    pub fn play_health(&self) {
        self.play(&self.health);
    }

    pub fn play_powerup(&self) {
        self.play(&self.powerup);
    }

    pub fn play_hack_start(&self) {
        self.play(&self.hack_start);
    }

    pub fn play_hack_blip(&self) {
        self.play(&self.hack_blip);
    }

    pub fn play_hack_success(&self) {
        self.play(&self.hack_success);
    }

    pub fn play_hack_fail(&self) {
        self.play(&self.hack_fail);
    }

    pub fn play_game_win(&self) {
        self.play(&self.game_win);
    }
}

// ============ WAV Generation ============

fn generate_wav(samples: &[f32]) -> Vec<u8> {
    let num_samples = samples.len();
    let data_size = num_samples * 2; // 16-bit samples
    let file_size = 36 + data_size;

    let mut wav = Vec::with_capacity(file_size + 8);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(file_size as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes()); // sample rate
    wav.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data_size as u32).to_le_bytes());

    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let int_sample = (clamped * 32767.0) as i16;
        wav.extend_from_slice(&int_sample.to_le_bytes());
    }

    wav
}

// ============ Sound Synthesis Primitives ============

fn sine_wave(freq: f32, duration: f32, volume: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin() * volume;
        samples.push(sample);
    }

    samples
}

fn noise_burst(duration: f32, volume: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for _ in 0..num_samples {
        let sample = (macroquad::rand::gen_range(-1.0f32, 1.0)) * volume;
        samples.push(sample);
    }

    samples
}

fn frequency_sweep(start_freq: f32, end_freq: f32, duration: f32, volume: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let mut phase = 0.0f32;

    for i in 0..num_samples {
        let t = i as f32 / num_samples as f32;
        let freq = start_freq + (end_freq - start_freq) * t;
        phase += freq * 2.0 * std::f32::consts::PI / SAMPLE_RATE as f32;
        let sample = phase.sin() * volume;
        samples.push(sample);
    }

    samples
}

fn apply_envelope(samples: &mut [f32], attack: f32, decay: f32) {
    let attack_samples = (SAMPLE_RATE as f32 * attack) as usize;
    let decay_samples = (SAMPLE_RATE as f32 * decay) as usize;
    let len = samples.len();

    // Attack
    for (i, sample) in samples.iter_mut().enumerate().take(attack_samples.min(len)) {
        let env = i as f32 / attack_samples as f32;
        *sample *= env;
    }

    // Decay
    if len > decay_samples {
        let decay_start = len - decay_samples;
        for (i, sample) in samples.iter_mut().enumerate().skip(decay_start) {
            let env = (len - i) as f32 / decay_samples as f32;
            *sample *= env;
        }
    }
}

fn mix(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().max(b.len());
    let mut result = vec![0.0; len];

    for (i, sample) in result.iter_mut().enumerate() {
        let sa = a.get(i).copied().unwrap_or(0.0);
        let sb = b.get(i).copied().unwrap_or(0.0);
        *sample = (sa + sb).clamp(-1.0, 1.0);
    }

    result
}

// ============ Sound Generators ============

fn generate_knife_swing() -> Vec<u8> {
    let noise = noise_burst(0.08, 0.4);
    let sweep = frequency_sweep(400.0, 150.0, 0.08, 0.3);
    let mut samples = mix(&noise, &sweep);
    apply_envelope(&mut samples, 0.005, 0.04);
    generate_wav(&samples)
}

fn generate_pistol_shot() -> Vec<u8> {
    let mut sine = sine_wave(180.0, 0.08, 0.5);
    let noise = noise_burst(0.03, 0.6);
    apply_envelope(&mut sine, 0.001, 0.06);
    let samples = mix(&sine, &noise);
    generate_wav(&samples)
}

fn generate_shotgun_blast() -> Vec<u8> {
    let mut low = sine_wave(80.0, 0.15, 0.6);
    let noise = noise_burst(0.1, 0.7);
    apply_envelope(&mut low, 0.001, 0.12);
    let samples = mix(&low, &noise);
    generate_wav(&samples)
}

fn generate_machine_pistol() -> Vec<u8> {
    let mut sine = sine_wave(350.0, 0.04, 0.4);
    let noise = noise_burst(0.02, 0.3);
    apply_envelope(&mut sine, 0.001, 0.03);
    let samples = mix(&sine, &noise);
    generate_wav(&samples)
}

fn generate_rifle_shot() -> Vec<u8> {
    let mut sine = sine_wave(150.0, 0.12, 0.5);
    let crack = noise_burst(0.02, 0.8);
    apply_envelope(&mut sine, 0.001, 0.1);
    let samples = mix(&sine, &crack);
    generate_wav(&samples)
}

fn generate_hit() -> Vec<u8> {
    let mut thud = sine_wave(120.0, 0.1, 0.5);
    let sweep = frequency_sweep(200.0, 80.0, 0.08, 0.3);
    apply_envelope(&mut thud, 0.001, 0.08);
    let samples = mix(&thud, &sweep);
    generate_wav(&samples)
}

fn generate_player_hit() -> Vec<u8> {
    let mut low = sine_wave(60.0, 0.15, 0.6);
    let mid = sine_wave(120.0, 0.1, 0.3);
    apply_envelope(&mut low, 0.001, 0.12);
    let samples = mix(&low, &mid);
    generate_wav(&samples)
}

fn generate_player_death() -> Vec<u8> {
    // Sad trombone / womp womp descending notes
    let note1 = sine_wave(311.0, 0.25, 0.5); // Eb4
    let note2 = sine_wave(277.0, 0.25, 0.5); // Db4
    let note3 = sine_wave(261.0, 0.25, 0.5); // C4
    let note4 = sine_wave(233.0, 0.5, 0.6); // Bb3 (longer, lower)

    let mut samples = Vec::new();
    samples.extend_from_slice(&note1);
    samples.extend_from_slice(&note2);
    samples.extend_from_slice(&note3);
    samples.extend_from_slice(&note4);
    apply_envelope(&mut samples, 0.02, 0.15);
    generate_wav(&samples)
}

fn generate_pickup() -> Vec<u8> {
    // Rising arpeggio: three quick notes
    let note1 = sine_wave(440.0, 0.06, 0.4);
    let note2 = sine_wave(554.0, 0.06, 0.4); // C#5
    let note3 = sine_wave(659.0, 0.08, 0.4); // E5

    let mut samples = Vec::new();
    samples.extend_from_slice(&note1);
    samples.extend_from_slice(&note2);
    samples.extend_from_slice(&note3);
    apply_envelope(&mut samples, 0.005, 0.03);
    generate_wav(&samples)
}

fn generate_health() -> Vec<u8> {
    let mut sweep = frequency_sweep(300.0, 600.0, 0.2, 0.4);
    apply_envelope(&mut sweep, 0.01, 0.1);
    generate_wav(&sweep)
}

fn generate_powerup() -> Vec<u8> {
    let sweep1 = frequency_sweep(200.0, 800.0, 0.25, 0.3);
    let sweep2 = frequency_sweep(250.0, 850.0, 0.25, 0.2);
    let mut samples = mix(&sweep1, &sweep2);
    apply_envelope(&mut samples, 0.01, 0.1);
    generate_wav(&samples)
}

fn generate_hack_start() -> Vec<u8> {
    // Alert beep sequence
    let beep1 = sine_wave(800.0, 0.08, 0.4);
    let pause = vec![0.0; (SAMPLE_RATE as f32 * 0.05) as usize];
    let beep2 = sine_wave(800.0, 0.08, 0.4);
    let beep3 = sine_wave(1000.0, 0.12, 0.5);

    let mut samples = Vec::new();
    samples.extend_from_slice(&beep1);
    samples.extend_from_slice(&pause);
    samples.extend_from_slice(&beep2);
    samples.extend_from_slice(&pause);
    samples.extend_from_slice(&beep3);
    generate_wav(&samples)
}

fn generate_hack_blip() -> Vec<u8> {
    let mut blip = sine_wave(600.0, 0.05, 0.3);
    apply_envelope(&mut blip, 0.005, 0.03);
    generate_wav(&blip)
}

fn generate_hack_success() -> Vec<u8> {
    // Victory arpeggio ascending
    let note1 = sine_wave(523.0, 0.1, 0.4); // C5
    let note2 = sine_wave(659.0, 0.1, 0.4); // E5
    let note3 = sine_wave(784.0, 0.15, 0.5); // G5
    let note4 = sine_wave(1047.0, 0.2, 0.5); // C6

    let mut samples = Vec::new();
    samples.extend_from_slice(&note1);
    samples.extend_from_slice(&note2);
    samples.extend_from_slice(&note3);
    samples.extend_from_slice(&note4);
    apply_envelope(&mut samples, 0.01, 0.1);
    generate_wav(&samples)
}

fn generate_hack_fail() -> Vec<u8> {
    // Descending harsh tone
    let sweep = frequency_sweep(600.0, 150.0, 0.4, 0.5);
    let noise = noise_burst(0.2, 0.3);
    let mut samples = mix(&sweep, &noise);
    apply_envelope(&mut samples, 0.01, 0.2);
    generate_wav(&samples)
}

fn generate_game_win() -> Vec<u8> {
    // Triumphant chord progression
    let c5 = sine_wave(523.0, 0.3, 0.3);
    let e5 = sine_wave(659.0, 0.3, 0.3);
    let g5 = sine_wave(784.0, 0.3, 0.3);

    let chord1 = mix(&mix(&c5, &e5), &g5);

    let c6 = sine_wave(1047.0, 0.5, 0.4);
    let e6 = sine_wave(1319.0, 0.5, 0.3);
    let g6 = sine_wave(1568.0, 0.5, 0.3);

    let chord2 = mix(&mix(&c6, &e6), &g6);

    let mut samples = Vec::new();
    samples.extend_from_slice(&chord1);
    samples.extend_from_slice(&chord2);
    apply_envelope(&mut samples, 0.02, 0.2);
    generate_wav(&samples)
}
