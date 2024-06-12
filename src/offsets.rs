use std::collections::HashMap;

impl RekordboxOffsets {
    pub fn default_version() -> &'static str {
        "7.0.1"
    }

    pub fn get_available_versions() -> HashMap<&'static str, RekordboxOffsets> {
        let mut map = HashMap::new();

        map.insert(
            "7.0.1",
            RekordboxOffsets {
                beat_baseoffset: 0x52EA410,
                decks: vec![vec![0x28, 0x0, 0x48], vec![0x28, 0x0, 0x50], vec![0x28, 0x8, 0x48], vec![0x28, 0x8, 0x50]],
                bar: vec![0x2468],
                beat: vec![0x246C],
                master_bpm: vec![0x0544A460, 0x28, 0x180, 0x0, 0x140],
                masterdeck_index: vec![0x052413A8, 0x20, 0x278, 0x124],
            }
        );

        map.insert(
            "6.8.5",
            RekordboxOffsets {
                beat_baseoffset: 0x0443F630,
                decks: vec![vec![0x278], vec![0x280], vec![0x288], vec![0x290]],
                bar: vec![0x18, 0x1e18],
                beat: vec![0x18, 0x1e1c],
                master_bpm: vec![0x04440240, 0x48, 0xF8, 0x28, 0xB98],
                masterdeck_index: vec![0x043DBDB0, 0x20, 0x270, 0xE20],
            }
        );

        map.insert(
            "6.8.4",
            RekordboxOffsets {
                beat_baseoffset: 0x0443F5D0,
                decks: vec![vec![0x120], vec![0x128]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x044401E0, 0x48, 0xF8, 0x28, 0xB98],
                masterdeck_index: vec![0x043DBD50, 0x20, 0x278, 0xE20],
            }
        );

        map.insert(
            "6.8.3",
            RekordboxOffsets {
                beat_baseoffset: 0x0443F650,
                decks: vec![vec![0x120], vec![0x128]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x04440260, 0x48, 0xF8, 0x28, 0xB98],
                masterdeck_index: vec![0x043DBDD0, 0x20, 0x278, 0xE20],
            }
        );

        map.insert(
            "6.8.2",
            RekordboxOffsets {
                beat_baseoffset: 0x043FB790,
                decks: vec![vec![0x120], vec![0x128]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x043FC3A0, 0x18, 0xF8, 0x0, 0x128],
                masterdeck_index: vec![0x04399C88, 0x20, 0x278, 0xe18],
            },
        );

        map.insert(
            "6.7.7",
            RekordboxOffsets {
                beat_baseoffset: 0x043BB250,
                decks: vec![vec![0x120], vec![0x128]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x043BBE60, 0x28, 0x208, 0x1d8, 0x140],
                masterdeck_index: vec![0x043BB250, 0x18, 0x720, 0x1058],
            },
        );

        map.insert(
            "6.7.4",
            RekordboxOffsets {
                beat_baseoffset: 0x04392560,
                decks: vec![vec![0x120], vec![0x128]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x0434c088, 0xe8, 0x1c0, 0x0, 0xb48],
                masterdeck_index: vec![0x04393430, 0x0, 0x58, 0x0, 0x530, 0x80, 0x144],
            },
        );

        map.insert(
            "6.7.3",
            RekordboxOffsets {
                beat_baseoffset: 0x043498e0,
                decks: vec![vec![0x118], vec![0x120]],
                bar: vec![0x1e18],
                beat: vec![0x1e1c],
                master_bpm: vec![0x0434A4F0, 0x18, 0x110, 0x0, 0x70, 0x158],
                masterdeck_index: vec![0x043498e0, 0x90, 0x19c],
            },
        );

        map
    }
}

#[derive(Clone)]
pub struct RekordboxOffsets {
    pub beat_baseoffset: usize,
    pub decks: Vec<Vec<usize>>,
    pub bar: Vec<usize>,
    pub beat: Vec<usize>,
    pub master_bpm: Vec<usize>,
    pub masterdeck_index: Vec<usize>,
}
