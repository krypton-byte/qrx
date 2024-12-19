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
    #[arg(short, long)]
    source: String,
    #[clap(value_hint = ValueHint::FilePath)]
    #[arg(short, long)]
    target: String,
    #[clap(value_hint = ValueHint::DirPath)]
    #[arg(short, long)]
    output: String,
}

fn main() {
    let parser = Args::parse();
    let img = image::open(parser.source).unwrap().to_luma8();
    let mut prepare_img = rqrr::PreparedImage::prepare(img);
    let grids = prepare_img.detect_grids();
    if grids.len() == 0{
        eprintln!("QR Object Not Detected");
        exit(1);
    }
    match grids[0].decode(){
        Ok((_, content))=>{
            let mut nodes = Nodes::from_str(&content).unwrap();
            let mut target = image::open(parser.target).unwrap();
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
                    let qr_modified = nodes_target.dumps();
                    let qrcode = QrCode::new(qr_modified).unwrap();
                    let result = qrcode.render::<Rgb<u8>>().max_dimensions(width, height).quiet_zone(false).build();
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