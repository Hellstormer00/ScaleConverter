use super::notes::Note;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ScaleCLI", about = "Converting scales and generating chords")]
pub struct Opt {
    /// The original scale to convert
    #[structopt(short, long, parse(from_str = crate::notes::Note::from_str))]
    pub scale: Note,

    /// Transpose the scale to this key
    #[structopt(short, long, parse(from_str = crate::notes::Note::from_str))]
    pub key: Option<Note>,
}
