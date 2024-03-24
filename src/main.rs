use std::sync::mpsc;

use anyhow::anyhow;
use midir::{MidiInput, MidiInputConnection};

// fn .........(a: u64, b: &[u8], c: &mut T)
// c will be data, BUT NOT YET
// There will come the day that c will be data. But today is not that day!
// (Actually it might be)
fn our_callback(a: u64, b: &[u8], tx: &mut mpsc::Sender<Vec<u8>>) {
    // Coming up next time: why are we sending Vec<u8> and not &[u8]? Also,
    // what is any of this?
    let _ = tx.send(b.to_vec());
}

// here's some source code inside anyhow:
//    type Result<T> = Result<T, anyhow::Error>;
// so when you say: anyhow::Result<T>
// it's as if you said: Result<T, anyhow::Error>

fn revive_mr_perkins() -> anyhow::Result<
    MidiInputConnection<
        // We're gonna send Vecs of u8s for now
        mpsc::Sender<Vec<u8>>,
    >,
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
        minput
            .connect(&port, "Braillest", our_callback, tx)
            .unwrap();
        return Ok(rx);
    }
    return Err(anyhow!(
        "We didn't find any ports. DID YOU PRESS THE PIANO KEY?!??"
    ));
}

fn main() {
    // Mr. Perkins is what's OK
    let mr_perkins = match revive_mr_perkins() {
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
    std::thread::sleep_ms(10000);
}
