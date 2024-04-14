use std::{
    fmt,
    fmt::{Debug, Formatter},
    io::Write,
    sync::mpsc, // Multi-Producer, Single Consumer (channel)
};

// For errors that the user vs program cares about?
use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use midir::{MidiInput, MidiInputConnection};

#[derive(Clone, Debug, ValueEnum)]
enum OutputMode {
    Dots,
    Letters,
    Both,
}

impl Default for OutputMode {
    fn default() -> Self {
        OutputMode::Both
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
struct Invocation {
    // -s (short)
    // --show (long)
    #[clap(short, long)]
    show: OutputMode,
}

#[derive(Debug)] // I am an attribute that applies to the thing below me
enum MidiEvent {
    NoteOn { note: u8 },
    NoteOff { note: u8 },
}

// enum is a shape sorter, and the items in it are what shapes can fit (or something)
enum BrailleKey {
    /// One of the "dot" keys. Pressing a dot key makes a dot. Releasing all
    /// held dot keys advances to the next cell.
    Dot(usize),
    /// The "space" key. Advance to the next cell. Beep if there is a dot key
    /// held down (instead of advancing).
    Space,
    /// The "enter" key. Advance to the beginning of the next line. Beep if
    /// there is a dot key held down (instead of advancing).
    Enter,
}

impl Debug for BrailleKey {
    fn fmt(&self, place_where_formatted_output_goes: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BrailleKey::Enter => write!(place_where_formatted_output_goes, "Enter"),
            BrailleKey::Space => write!(place_where_formatted_output_goes, "Space"),
            BrailleKey::Dot(n) => write!(
                place_where_formatted_output_goes,
                "Dot({})={}=|{}|",
                n,
                n + 1,
                // << means "bit shift left"
                // can also be read as "multiply by two N times"
                char::from_u32(0x2800 + (1 << n)).unwrap()
            ),
        }
    }
}

impl BrailleKey {
    fn from_midi_note(note: u8) -> Option<BrailleKey> {
        match note {
            48 => Some(BrailleKey::Dot(2)),
            50 => Some(BrailleKey::Dot(1)),
            52 => Some(BrailleKey::Dot(0)),
            60 => Some(BrailleKey::Dot(3)),
            62 => Some(BrailleKey::Dot(4)),
            64 => Some(BrailleKey::Dot(5)),
            _ => {
                if note > 66 {
                    Some(BrailleKey::Enter)
                } else if note < 46 {
                    Some(BrailleKey::Enter)
                } else if (53..=59).contains(&note) {
                    Some(BrailleKey::Space)
                } else {
                    None
                }
            }
        }
    }
}

// fn .........(a: u64, b: &[u8], c: &mut T)
fn our_callback(_timestamp: u64, data: &[u8], tx: &mut mpsc::Sender<MidiEvent>) {
    // The first byte of a MIDI command has four bits of Command and four bits
    // of cHannel:
    //
    // CCCCHHHH
    //
    // Shift right by four to get the Command:
    // 0000CCCC
    //
    // Bitwise AND by 0xF=0b1111 to get the cHannel:
    //   CCCCHHHH
    // & 00001111
    // = 0000HHHH
    let command = data[0] >> 4;
    let _channel = data[0] & 0xF;
    match command {
        8 => {
            // Note off!
            let _ = tx.send(MidiEvent::NoteOff { note: data[1] });
        }
        9 => {
            // Note on or off(?!)!
            let note = data[1];
            let velocity = data[2];
            if velocity == 0 {
                // it is a note off
                let _ = tx.send(MidiEvent::NoteOff { note });
            } else {
                // it is a note on
                let _ = tx.send(MidiEvent::NoteOn { note });
            }
        }
        _ => {
            // We don't care about this command!
        }
    }
}

// here's some source code inside anyhow:
//    type Result<T> = Result<T, anyhow::Error>;
// so when you say: anyhow::Result<T>
// it's as if you said: Result<T, anyhow::Error>

fn revive_mr_perkins() -> anyhow::Result<
    // We're gonna send Vecs of u8s for now
    (
        MidiInputConnection<mpsc::Sender<MidiEvent>>,
        mpsc::Receiver<MidiEvent>,
    ),
> {
    // unwrap = blow up and die (if anything goes wrong)
    let minput = MidiInput::new("Mister Perkins Himself").unwrap();
    // something something ports?
    eprintln!("Something something ports?");
    for port in minput.ports().iter() {
        eprintln!("Port! It is: {:?}", minput.port_name(port));
        // M.P.S.C.: Multi[ple] Producer, Single Consumer.
        let (tx, rx) = mpsc::channel();
        // something something connect
        let the_connection = minput
            .connect(&port, "Braillest", our_callback, tx)
            .unwrap();
        return Ok((the_connection, rx));
    }
    return Err(anyhow!(
        "We didn't find any ports. DID YOU PRESS THE PIANO KEY?!??"
    ));
}

fn main() {
    dbg!(Invocation::parse());
    // Mr. Perkins is what's OK
    let (_mr_perkins_house, mr_perkins) = match revive_mr_perkins() {
        Ok(x) => x,
        Err(error_message) => {
            // voodoo to make bold purple error messages w/ new lines on either side
            // \x1b = "escape"
            // \x1b[ = "start an ANSI control sequence!"
            // (numbers with semicolons in) = "parameters to ANSI control sequence"
            // m = This is a Set Graphics Rendition command! those numbers are Graphics Renditions!
            // 0 = turn it all off!!!!!!
            // 1 = turn on bold!
            // 3x = set the foreground color to x
            //   0: black (dark gray)
            //   1: red
            //   2: green
            //   3: yellow
            //   4: blue
            //   5: magenta
            //   6: cyan
            //   7: white (light gray)
            // 4x = set the background color to x
            eprintln!("\n\x1b[1;35;43m {error_message} \x1b[0m\n");
            std::process::exit(1);
        }
    };
    // Dots that are present in this cell of the brailler
    let mut dots_present = 0;
    // Dot keys that are *currently being held down*
    let mut dots_held = 0;
    // next time: talk about what the heck this syntax is
    while let Ok(message) = mr_perkins.recv() {
        match message {
            MidiEvent::NoteOn { note } => {
                // what to do if there's a note on
                // later: also talk about what the heck THIS syntax is, because
                // it's related
                let Some(key) = BrailleKey::from_midi_note(note) else {
                    continue;
                };
                match key {
                    BrailleKey::Dot(dot) => {
                        // Make the dot present and held
                        dots_present |= (1 << dot);
                        dots_held |= (1 << dot);
                        print!(
                            // U+0008 = backspace!
                            "{}\x08",
                            char::from_u32(0x2800 + dots_present as u32).unwrap()
                        );
                        let _ = std::io::stdout().flush();
                    }
                    BrailleKey::Space => {
                        // If any dots are held, beep.  Otherwise, space.
                        if dots_held != 0 {
                            print!("\x07"); // ASCII BEL[L]
                        } else {
                            print!(" ");
                        }
                        let _ = std::io::stdout().flush();
                    }
                    BrailleKey::Enter => {
                        // If any dots are held, beep.  Otherwise, enter.
                        if dots_held != 0 {
                            print!("\x07"); // ASCII BEL[L]  btw it is an L not a 1 or an I
                        } else {
                            print!("\n");
                        }
                        // The time of out is Now.
                        let _ = std::io::stdout().flush();
                    }
                }
            }
            MidiEvent::NoteOff { note } => {
                // what to do if there's a note off
                let Some(key) = BrailleKey::from_midi_note(note) else {
                    continue;
                };
                match key {
                    BrailleKey::Dot(dot) => {
                        //      1
                        // ( << dot, shift left by dot, which is currently 2)
                        //    100
                        // (apply the ! meaning NOT)
                        // 111011
                        // (now we can bitwise and the dots_held to make that
                        // dot not be held)
                        dots_held &= !(1 << dot);
                        if dots_held == 0 {
                            print!("{}", char::from_u32(0x2800 + dots_present as u32).unwrap());
                            dots_present = 0;
                            let _ = std::io::stdout().flush();
                        }
                    }
                    _ => {} // but also maybe consider doing something (later!)
                }
            }
        }
    }
}
