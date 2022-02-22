use rand::prelude::*;

const UNALTERED_ROOT: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
const ALTERED_ROOT: [&str; 10] = ["Ab", "A#", "Bb", "C#", "Db", "D#", "Eb", "F#", "Gb", "G#"];

const BASIC_TRIAD: [&[&str]; 4] = [
    /* Major */ &["", "M", "Δ"],
    /* Minor */ &["m", "-"],
    /* Suspended 4th */ &["sus4", "sus"],
    /* Suspended 2nd */ &["sus2"],
];
const ADVANCED_TRIAD: [&[&str]; 2] = [/* Augmented */ &["+"], /* Diminished */ &["o"]];

const BASIC_SEVENTH: [&[&str]; 2] = [
    /* Major */ &["M7", "Δ7"],
    /* Minor */ &["m7", "-7"],
];
const ADVANCED_SEVENTH: [&[&str]; 3] = [
    /* Dominant */ &["7"],
    /* Half-diminished */ &["m7b5", "-7b5", "ø", "ø7", "m7o5", "-7o5"],
    /* Diminished */ &["o7"],
];
const RARE_SEVENTH: [&[&str]; 4] = [
    /* Minor-major */ &["mM7", "m#7", "-M7", "-Δ7", "-Δ"],
    /* Augmented major */ &["+M7", "+Δ", "M7#5", "M7+5", "Δ7#5", "Δ7+5"],
    /* Augmented */ &["+7", "7#5", "7+5"],
    /* Seventh flat five */ &["7b5"],
];

const ENABLE_ALTERED_ROOT: bool = true;
const ENABLE_ADVANCED_TRIAD: bool = false;
const ENABLE_BASIC_SEVENTH: bool = true;
const ENABLE_ADVANCED_SEVENTH: bool = true;
const ENABLE_RARE_SEVENTH: bool = false;

const MAX_CHARS_PER_LINE: usize = 40;
const NUM_LINES: usize = 19 * 2;

fn main() {
    let mut roots = UNALTERED_ROOT.into_iter().collect::<Vec<_>>();
    if ENABLE_ALTERED_ROOT {
        roots.extend_from_slice(&ALTERED_ROOT[..]);
    }

    let mut chords = BASIC_TRIAD.into_iter().collect::<Vec<_>>();
    if ENABLE_ADVANCED_TRIAD {
        chords.extend_from_slice(&ADVANCED_TRIAD[..]);
    }
    if ENABLE_BASIC_SEVENTH {
        chords.extend_from_slice(&BASIC_SEVENTH[..]);
    }
    if ENABLE_ADVANCED_SEVENTH {
        chords.extend_from_slice(&ADVANCED_SEVENTH[..]);
    }
    if ENABLE_RARE_SEVENTH {
        chords.extend_from_slice(&RARE_SEVENTH[..]);
    }

    let max_root_length = roots.iter().map(|s| s.chars().count()).max().unwrap();
    let max_chord_length = max_root_length
        + chords
            .iter()
            .flat_map(|a| a.iter())
            .map(|s| s.chars().count())
            .max()
            .unwrap();
    let chord_spacing = (max_chord_length / 2).max(2);
    let chord_chars = max_chord_length + chord_spacing;
    let chords_per_line = MAX_CHARS_PER_LINE / chord_chars;

    let mut rng = rand::thread_rng();
    for _ in 0..NUM_LINES {
        for chord in 0..chords_per_line {
            let root = roots.choose(&mut rng).unwrap();
            let chord_type = chords.choose(&mut rng).unwrap();
            let chord_symbol = chord_type.choose(&mut rng).unwrap();
            print!("{}{}", root, chord_symbol);

            if chord != chords_per_line - 1 {
                let padding_chars =
                    chord_chars - root.chars().count() - chord_symbol.chars().count();
                for _ in 0..padding_chars {
                    print!(" ");
                }
            }
        }
        println!();
    }
}
