extern crate bytecodec;
extern crate flv_codec;
#[macro_use]
extern crate trackable;

use bytecodec::io::{IoDecodeExt, ReadBuf};
use bytecodec::Decode;
use flv_codec::FileDecoder;
use trackable::error::MainError;

fn main() -> Result<(), MainError> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut buf = ReadBuf::new(vec![0; 1024]);
    let mut decoder = FileDecoder::new();
    let mut is_header_shown = false;

    while !buf.stream_state().is_eos() {
        track!(buf.fill(&mut input))?;
        track!(decoder.decode_from_read_buf(&mut buf))?;
        if let Some(h) = decoder.header() {
            if !is_header_shown {
                println!("[header]");
                println!("has_audio = {}", h.has_audio);
                println!("has_video = {}", h.has_video);
                println!("");
                is_header_shown = true;
            }
        }
        if decoder.is_idle() {
            let tag = track!(decoder.finish_decoding())?;
            println!("[[tags]]");
            println!("type = {:?}", tag.tag_type());
            println!("timestamp = {:?}", tag.timestamp());
            println!("stream_id = {:?}", tag.stream_id());
            println!("");
        }
    }

    Ok(())
}
