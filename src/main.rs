extern crate image;
extern crate clap;
extern crate rand;
extern crate itertools;

use clap::*;

mod codec;

mod rgba;

mod utils;

fn main()
{
    let matches = App::new("stag")
        .version("0.0.1")
        .about("Image steganography")
        .subcommand(SubCommand::with_name("encode")
            .about("encodes a file")
            .arg(Arg::with_name("mode")
                 .short("m")
                 .long("mode")
                 .value_name("MODE")
                 .help("Set the encoding mode, default depends on SOURCE type")
                 .takes_value(true))
            .arg(Arg::with_name("SOURCE")
                 .help("The image source")
                 .index(1)
                 .required(true))
            .arg(Arg::with_name("OUTPUT")
                 .help("The output image")
                 .index(2)
                 .required(true)))
        .subcommand(SubCommand::with_name("decode")
            .about("decodes a file")
            .arg(Arg::with_name("mode")
                 .short("m")
                 .long("mode")
                 .value_name("MODE")
                 .help("Set the decoding mode, default depends on SOURCE type")
                 .takes_value(true))
            .arg(Arg::with_name("SOURCE")
                 .help("The image source")
                 .index(1)
                 .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encode")
    {
        encode(
            matches.value_of("mode"),
            matches.value_of("SOURCE").unwrap(),
            matches.value_of("OUTPUT").unwrap()
        );
    }
    
    if let Some(matches) = matches.subcommand_matches("decode")
    {
        decode(
            matches.value_of("mode"),
            matches.value_of("SOURCE").unwrap()
        );
    }
}

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

use image::{open, DynamicImage};
use rand::StdRng;

use rgba::{encode_rgba, decode_rgba, RgbaMode};

fn encode(mode: Option<&str>, source: &str, output: &str)
{
    let dyimage = match open(source)
    {
        Ok(di) => di,
        Err(_) =>
        {
            eprintln!("Error opening source image");
            ::std::process::exit(1);
        },
    };

    match dyimage
    {
        DynamicImage::ImageRgba8(mut image) =>
        {
            let mode = match mode
            {
                Some("alpha") => RgbaMode::Alpha,
                Some("all") => RgbaMode::All,
                Some(_) | None => RgbaMode::Alpha,
            };

            let rng = match StdRng::new()
            {
                Ok(r) => r,
                Err(_) =>
                {
                    eprintln!("Error creating an RNG");
                    std::process::exit(1);
                }
            };

            let mut payload = String::new();
            stdin().read_to_string(&mut payload).unwrap();

            encode_rgba(&mut image, &payload.bytes().collect::<Vec<_>>(), mode, rng);

            match image.save(output)
            {
                Ok(_) => {},
                Err(_) =>
                {
                    eprintln!("Error saving output file");
                    ::std::process::exit(1);
                }
            };
        },
        _ => unimplemented!()
    }
}

fn decode(mode: Option<&str>, source: &str)
{
    let dyimage = match open(source)
    {
        Ok(di) => di,
        Err(_) =>
        {
            eprintln!("Error opening source image");
            ::std::process::exit(1);
        },
    };

    match dyimage
    {
        DynamicImage::ImageRgba8(image) =>
        {
            let mode = match mode
            {
                Some("alpha") => RgbaMode::Alpha,
                Some("all") => RgbaMode::All,
                Some(_) | None => RgbaMode::Alpha,
            };

            let payload = decode_rgba(&image, mode);

            match stdout().write(&payload)
            {
                Ok(_) => {},
                Err(e) =>
                {
                    eprintln!("Error writing payload: {}", e.description());
                    ::std::process::exit(1);
                }
            };
        },
        _ => unimplemented!()
    }
}
