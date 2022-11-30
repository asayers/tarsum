use bytesize::ByteSize;
use clap::Parser;
use sha1::Digest;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let archive = tar::Archive::new(File::open(opts.file)?);
    for (hdr, mut slice) in fileslice::slice_tarball(archive)? {
        if hdr.entry_type() == tar::EntryType::Regular {
            let mut data = vec![];
            slice.read_to_end(&mut data)?;
            println!(
                "{}\t{:>8} {:x}",
                hdr.path()?.display(),
                ByteSize::b(hdr.size()?).to_string(),
                sha1::Sha1::digest(&data)
            );
        }
    }
    Ok(())
}
