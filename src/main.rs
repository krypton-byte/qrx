use std::cmp;
use std::process::exit;

use clap::{Parser, ValueHint};
use qrcode::QrCode;
use qris::node::Nodes;
use image::{DynamicImage, Rgb};
use rqrr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(value_hint = ValueHint::FilePath)]
    #[arg(short, long, help = "Path to the source QRIS file to copy data from.")]
    source: String,

    #[clap(value_hint = ValueHint::FilePath)]
    #[arg(short, long, help = "Path to the target QRIS file to overwrite with source data.")]
    target: String,

    #[clap(value_hint = ValueHint::DirPath)]
    #[arg(short, long, help = "Path to the directory where the output QRIS file will be saved.")]
    output: String,

    #[arg(long, default_value_t = false, help = "Use the target QRIS's frame to create a new QRIS image with the source's information. When enabled, generates a plain QR code with the source's information instead.")]
    raw: bool,

    #[arg(long, default_value_t = 500, help = "Set the canvas size for the QRIS image in pixels (default: 500).")]
    size: u32,
}

fn main() {
    let parser = Args::parse();
    let img = image::open(parser.source);
    let img = match img {
        Ok(r)=>r,
        Err(e) => {
            eprintln!("Err: {}", &e);
            exit(1);
        }
    }.to_luma8();
    let mut prepare_img = rqrr::PreparedImage::prepare(img);
    let grids = prepare_img.detect_grids();
    if grids.len() == 0{
        eprintln!("QR Object Not Detected");
        exit(1);
    }
    match grids[0].decode(){
        Ok((_, content))=>{
            let mut nodes = Nodes::from_str(&content).unwrap();
            let target = image::open(parser.target);
            let mut target = match target {
                Ok(r)=>r,
                Err(e) => {
                    eprintln!("Err: {}", &e);
                    exit(1);
                }
            };
            let target_img = target.to_luma8();
            let mut prepare_target_img = rqrr::PreparedImage::prepare(target_img);
            let grid_target = prepare_target_img.detect_grids();
            if grid_target.len() == 0 {
                eprintln!("QR Target Object Not Detected");
                exit(1);
            }
            match grid_target[0].decode() {
                Ok((_, content_target)) => {
                    let bounds = grid_target[0].bounds;
                    let width = (cmp::max(bounds[1].x, bounds[2].x) - cmp::min(bounds[0].x, bounds[3].x)) as u32;
                    let height = (cmp::max(bounds[2].y, bounds[3].y) - cmp::min(bounds[0].y, bounds[1].y)) as u32;
                    let nodes_target = Nodes::from_str(&content_target).unwrap();
                    nodes.set_merchant_city(nodes_target.get_merchant_city().unwrap().to_string());
                    nodes.set_merchant_name(nodes_target.get_merchant_name().unwrap().to_string());
                    nodes.set_postal_code(nodes_target.get_postal_code().unwrap().to_string());
                    nodes.rewrite_crc16();
                    let qr_modified = nodes.dumps();
                    let qrcode = QrCode::new(qr_modified).unwrap();
                    let result = qrcode.render::<Rgb<u8>>().max_dimensions(if parser.raw{parser.size}else{width}, if parser.raw{parser.size}else{height}).quiet_zone(parser.raw).build();
                    if parser.raw {
                        match result.save(&parser.output) {
                            Ok(_) => {
                                println!("qr saved as {}", parser.output);

                            },
                            Err(e)=>{
                                println!("saving image failure: {}", e);

                            }
                        }
                    }else{
                        let result_img = DynamicImage::ImageRgb8(result).resize(width, height, image::imageops::FilterType::Gaussian);
                        let start = grid_target[0].bounds[0];
                        image::imageops::overlay(&mut target, &result_img, start.x.into(), start.y.into());
                        let save = target.save(&parser.output);
                        match save {
                            Ok(_) => {
                                println!("qr saved as {}", parser.output);
                            },
                            Err(e) => {
                                println!("saving image failure: {}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        },
        Err(r)=>{
            eprint!("Error: {}", r);
            exit(1);
        }
    }
}