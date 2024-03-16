use midir::MidiInput;

fn main() {
    // unwrap = blow up and die (if anything goes wrong)
    let minput = MidiInput::new("Mister Perkins Himself").unwrap();
    // something something ports?
    println!("Something something ports?");
    for port in minput.ports().iter() {
        println!("Port! It is: {:?}", minput.port_name(port));
    }
    println!("That is the end of the ports.");
    // something something connect?
}
