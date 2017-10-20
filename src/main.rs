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
                 .required(true))
            .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .help("the amount of bytes to decode")
                 .required(true)))
        .subcommand(SubCommand::with_name("estimate")
                .about("estimate how many bytes will fit into a file")
                .arg(Arg::with_name("SOURCE")
                     .help("The image source")
                     .index(1)
                     .takes_value(true))
                .arg(Arg::with_name("mode")
                     .short("m")
                     .long("mode")
                     .value_name("MODE")
                     .help("The encoding mode, default depends on SOURCE type")
                     .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encode")
    {
        dispatch_encode(
            matches.value_of("mode"),
            matches.value_of("SOURCE").unwrap(),
            matches.value_of("OUTPUT").unwrap()
        );
    }
    
    if let Some(matches) = matches.subcommand_matches("decode")
    {
        dispatch_decode(
            matches.value_of("mode"),
            matches.value_of("SOURCE").unwrap(),
            matches.value_of("length").unwrap()
        );
    }

    if let Some(matches) = matches.subcommand_matches("estimate")
    {
        dispatch_estimate(
            matches.value_of("mode"),
            matches.value_of("SOURCE").unwrap()
        );
    }
}

use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;

use image::{open, DynamicImage};
use rand::StdRng;

use codec::Codec;
use rgba::RgbaCodec;

fn dispatch_encode(mode: Option<&str>, source: &str, output: &str)
{
    let dyimage = match open(source)
    {
        Ok(di) => di,
        Err(_) => error_out("Error opening source image for encoding")
    };

    match dyimage
    {
        DynamicImage::ImageRgba8(image) =>
        {
            match encode::<RgbaCodec>(image, mode).save(output)
            {
                Ok(_) => {},
                Err(_) => error_out("Error saving encoded output file"),
            };
        },
        _ => error_out("Unsupported filetype"),
    }
}

fn dispatch_decode(mode: Option<&str>, source: &str, len: &str)
{
    let len = match len.parse::<usize>()
    {
        Ok(l) => l,
        Err(_) => error_out("len argument to decode is not a number"),
    };

    let dyimage = match open(source)
    {
        Ok(di) => di,
        Err(_) => error_out("Error opening source image for decoding"),
    };

    match dyimage
    {
        DynamicImage::ImageRgba8(image) =>
        {
            decode::<RgbaCodec>(image, mode, len);
        },
        _ => error_out("Unsupported filetype"),
    }
}

fn dispatch_estimate(mode: Option<&str>, source: &str)
{
    let dyimage = match open(source)
    {
        Ok(di) => di,
        Err(_) => error_out("Error opening source image for estimating"),
    };

    match dyimage
    {
        DynamicImage::ImageRgba8(image) =>
        {
            estimate::<RgbaCodec>(image, mode);
        },
        _ => error_out("Unsupported filetype"),
    }
}

fn encode<C: Codec>(mut image: C::Input, mode: Option<&str>) -> C::Input
{
    let mode = if let Some(mode) = mode
    {
        match C::Mode::from_str(mode)
        {
            Ok(mode) => mode,
            Err(_) => error_out("Error parsing mode string"),
        }
    }
    else
    {
        C::Mode::default()
    };

    let rng = match StdRng::new()
    {
        Ok(r) => r,
        Err(_) => error_out("Error creating source of randomness"),
    };

    let mut payload = String::new();
    stdin().read_to_string(&mut payload).unwrap();

    C::encode(&mut image, payload.as_bytes(), mode, rng);

    image
}

fn decode<C: Codec>(image: C::Input, mode: Option<&str>, len: usize)
{
    let mode = if let Some(mode) = mode
    {
        match C::Mode::from_str(mode)
        {
            Ok(mode) => mode,
            Err(_) => error_out("Error parsing mode string"),
        }
    }
    else
    {
        C::Mode::default()
    };

    let mut buf = vec![0; len];

    C::decode(&image, &mut buf, len, mode);

    match stdout().write(&buf)
    {
        Ok(_) => {},
        Err(_) => error_out("Error writing decoded payload"),
    };
}

fn estimate<C: Codec>(image: C::Input, mode: Option<&str>)
{
    let mode = if let Some(mode) = mode
    {
        match C::Mode::from_str(mode)
        {
            Ok(mode) => mode,
            Err(_) => error_out("Error parsing mode string"),
        }
    }
    else
    {
        C::Mode::default()
    };

    match C::estimate(&image, mode)
    {
        Some(i) => println!("Estimate {} bytes", i),
        None => println!("Could not make an estimate"),
    }
}

fn error_out(msg: &str) -> !
{
    eprintln!("{}", msg);
    ::std::process::exit(1)
}
