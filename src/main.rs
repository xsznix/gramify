pub mod grams;
pub mod reducers;

use getopts::Options;
use memmap::Mmap;
use regex::Regex;
use serde::Serialize;
use std::env;
use std::io::Read;
use std::fs::File;

enum OutputFormat {
  Json,
  Msgpack,
}

fn main() {
  // Get options
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.optflag("h", "help", "Show usage instructions, then exit");
  opts.optopt("o", "output-format", "Output format", "json|msgpack");
  opts.optopt("i", "input-format", "Input format", "json|msgpack|raw");
  opts.optopt("", "letter-threshold", "Threshold of significance for letters. Letters that appear fewer than NUM times per million will not appear in the output.", "NUM");
  opts.optopt("", "letter-pattern", "Regex pattern for letters. Letters that don't match REGEX will be excluded from output.", "REGEX");
  opts.optopt("", "bigram-threshold", "Threshold of significance for bigrams. Bigrams that appear fewer than NUM times per million will not appear in the output.", "NUM");
  opts.optopt("", "bigram-pattern", "Regex pattern for bigrams. Bigrams that don't match REGEX will be excluded from output.", "REGEX");
  opts.optopt("", "skipgram-threshold", "Threshold of significance for skipgrams. Skipgrams that appear fewer than NUM times per million will not appear in the output.", "NUM");
  opts.optopt("", "skipgram-pattern", "Regex pattern for skipgrams. Skipgrams that don't match REGEX will be excluded from output.", "REGEX");
  opts.optopt("", "trigram-threshold", "Threshold of significance for trigrams. Trigrams that appear fewer than NUM times per million will not appear in the output.", "NUM");
  opts.optopt("", "trigram-pattern", "Regex pattern for trigrams. Trigrams that don't match REGEX will be excluded from output.", "REGEX");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(e) => panic!("{}", e),
  };
  if matches.opt_present("help") || matches.free.len() == 0 {
    print_usage(&opts, &program);
    return;
  }
  let output_format = match matches.opt_str("output-format").unwrap().as_str() {
    "json" => OutputFormat::Json,
    "msgpack" => OutputFormat::Msgpack,
    _ => {
      print_usage(&opts, &program);
      return;
    }
  };

  // Read input file
  let mut file = match File::open(&matches.free[0]) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
  };
  let mut grams = match matches.opt_str("input-format").unwrap().as_str() {
    "raw" => {
      let corpus = unsafe {
        match Mmap::map(&file) {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        }
      };
      grams::make_grams(&corpus)
    }
    "json" => {
      let mut contents = String::new();
      match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
      };
      match serde_json::from_str(&contents) {
        Ok(grams) => grams,
        Err(e) => panic!("{}", e),
      }
    }
    "msgpack" => {
      match rmp_serde::from_read(file) {
        Ok(grams) => grams,
        Err(e) => panic!("{}", e),
      }
    }
    _ => {
      print_usage(&opts, &program);
      return;
    }
  };

  // Apply thresholds of significance and patterns
  if let Ok(Some(threshold)) = matches.opt_get::<f64>("letter-threshold") {
    grams.apply_letter_threshold(threshold);
  }
  if let Some(p) = matches.opt_str("letter-pattern") {
    let pattern = Regex::new(&p).unwrap();
    grams.apply_letter_pattern(pattern);
  }
  if let Ok(Some(threshold)) = matches.opt_get::<f64>("bigram-threshold") {
    grams.apply_bigram_threshold(threshold);
  }
  if let Some(p) = matches.opt_str("bigram-pattern") {
    let pattern = Regex::new(&p).unwrap();
    grams.apply_bigram_pattern(pattern);
  }
  if let Ok(Some(threshold)) = matches.opt_get::<f64>("skipgram-threshold") {
    grams.apply_skipgram_threshold(threshold);
  }
  if let Some(p) = matches.opt_str("skipgram-pattern") {
    let pattern = Regex::new(&p).unwrap();
    grams.apply_skipgram_pattern(pattern);
  }
  if let Ok(Some(threshold)) = matches.opt_get::<f64>("trigram-threshold") {
    grams.apply_trigram_threshold(threshold);
  }
  if let Some(p) = matches.opt_str("trigram-pattern") {
    let pattern = Regex::new(&p).unwrap();
    grams.apply_trigram_pattern(pattern);
  }

  // Write output
  match output_format {
    OutputFormat::Json => {
      let result = serde_json::to_string(&grams);
      if let Ok(result) = result {
        println!("{}", result);
      }
    }
    OutputFormat::Msgpack => {
      let mut serializer = rmp_serde::Serializer::new(std::io::stdout());
      match grams.serialize(&mut serializer) {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
      }
    }
  }
}

fn print_usage(opts: &Options, program: &str) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}