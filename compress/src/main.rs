// GzEncoder is struct -> compress the data using Gzip format
use flate2::write::GzEncoder;

// Compression is enum 
// -> fast() fast compression but less effective
// -> best() slower but achieves maximum compression
// -> default() balace between speed and compression ratio
use flate2::Compression;

// args -> arguments passed to the program like cargo run index.html 
// then args.nth is how i access it 
// args
use std::env::args;

// File sturct -> File handling opening and reading files creating
use std::fs::File;

// Copy function-> which is used to copy data from one  reader to a writer
use std::io::copy;

// BufReader struct -> 
use std::io::BufReader;
// BufReader is a wrapper around the reader -> Read trait 

// Why Use BufReader?
// When reading data from sources like files or network streams, direct reads can be slow because they often involve multiple system calls. BufReader minimizes this overhead by reading chunks of data into an internal buffer. Subsequent read operations fetch data from this buffer, making them faster and more efficient.


// Istant struct -> measure elapsed time

use std::time::Instant;

fn main() {
    // println!("Hello, world!");
    

    // cargo run book.pdf compress_book.pdf 
    // cargo is terminal crate 
    // run is first aurments 
    // if args().len() !=3 {
    //     eprintln!("Usage: `source` `target`");
    //     return;
    // }
    // it take aurments from the command line
    if args().len() !=4 {
        eprintln!("Usage: `type` `source` `target`");
        return;
    }

    // input we are nameing the input file input 
    // BufReader is reading the input file 
    // File::open() -> trying to access the file in root repositatry
    let mut input = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());

    // we are creating the output file
    let output = File::create(args().nth(3).unwrap()).unwrap();


    let mut encoder;

    // BnEncoder is making the new copressing file
    if args().nth(1).unwrap() == "best" {
        encoder = GzEncoder::new(output, Compression::best());
    } else if args().nth(1).unwrap() == "fast" {
        encoder = GzEncoder::new(output, Compression::fast());
    } else{
        encoder = GzEncoder::new(output, Compression::default());
    }

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    
    // println!("")
    println!("Source len:{:?}", input.get_ref().metadata().unwrap().len());

    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elaspesed :{:?}", start.elapsed());

}
