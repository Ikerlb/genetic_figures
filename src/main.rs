mod state;
mod rectangle;
mod util;
mod figure;
mod minimize;
mod ellipse;
mod ga;
mod img;
mod scanline;
mod bezier;
mod line;

use image::{imageops,FilterType,ImageBuffer};
use time::PreciseTime;
use clap::{Arg, App};
use state::State;
use std::fs::File;

const SIZE: usize = 256;

fn main() {
    let matches = App::new("figures")
                    .version("1.0")
                    .author("iker lissarrague. <ikerlb95@gmail.com>")
                    .about("approximate image with geometric figures")
                    .arg(Arg::with_name("inputFile")
                        .long("input")
                        .short("i")
                        .required(true)
                        .help("specify image to approximate")
                        .takes_value(true))
                    .arg(Arg::with_name("outputFile")
                        .long("output")
                        .short("o")
                        .required(true)
                        .help("specify output file")
                        .takes_value(true))
                    .arg(Arg::with_name("numberOfFigures")
                        .short("n")
                        .required(true)
                        .help("number of figures to approximate image")
                        .takes_value(true))
                    .arg(Arg::with_name("alpha")
                        .short("a")
                        .long("alpha")
                        .required(false)
                        .help("alpha value for the figures. 1-255")
                        .takes_value(true))
                    .arg(Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .required(false)
                        .help("width of output image")
                        .takes_value(true))
                    .arg(Arg::with_name("height")
                         .short("h")
                         .long("height")
                         .required(false)
                         .help("height of output image")
                         .takes_value(true))
                    .arg(Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .required(true)
                        .help("figure mode. 1 is for rectangles (default)")
                        .takes_value(true))
                    .arg(Arg::with_name("population")
                        .short("p")
                        .long("population")
                        .required(false)
                        .takes_value(true)
                        .help("population size. defaults to 1000"))
                    .arg(Arg::with_name("generations")
                        .short("g")
                        .long("generations")
                        .required(false)
                        .takes_value(true)
                        .help("number of generations per figure. defaults to 100."))
                    .arg(Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .required(false)
                        .help("runs program in verbose mode"))
                    .arg(Arg::with_name("cairo")
                         .short("c")
                         .long("cairo")
                         .required(false)
                         .help("output image using cairo"))
                    .arg(Arg::with_name("sweep")
                         .long("sweep")
                         .required(false)
                         .help("hill descent best solution from genetic algorithm"))
                .get_matches();

    let input_file=matches.value_of("inputFile").unwrap();
    let output_file=matches.value_of("outputFile").unwrap();
    let alpha:u8=matches.value_of("alpha")
                        .unwrap_or("127")
                        .parse::<u8>()
                        .unwrap();
    let n=matches.value_of("numberOfFigures")
                 .unwrap()
                 .parse::<u32>()
                 .unwrap();
    let mode=matches.value_of("mode")
                   .unwrap_or("1")
                   .parse::<u32>()
                   .unwrap();
    let p=matches.value_of("population")
                 .unwrap_or("1000")
                 .parse::<usize>()
                 .unwrap();
    let g=matches.value_of("generations")
                 .unwrap_or("100")
                 .parse::<u32>()
                 .unwrap();
    let sweep=matches.is_present("sweep");
    let resize=SIZE as u32;
    let verbose=matches.is_present("verbose");
    let use_cairo=matches.is_present("cairo");
    let t=image::open(input_file).unwrap().to_rgba();
    let (w,h)=t.dimensions();
    let output_width=match matches.value_of("width"){
                        Some(s) => s.parse::<u32>().unwrap(), 
                        None => w,
                     };
    let output_height=match matches.value_of("height"){
                        Some(s) => s.parse::<u32>().unwrap(),
                        None => h,
                     };
    let ftn=FilterType::Nearest;
    let ftl=FilterType::Lanczos3;
    let target=imageops::resize(&t,resize,resize,ftl);
    let mut state=State::new(target,alpha,output_width as i32,output_height as i32);
    println!("Starting...");
    let start=PreciseTime::now();
    for i in 0..n{
        let fstart=PreciseTime::now();
        let c=state.step(g,p,mode,sweep);
        let fend=PreciseTime::now();
        let dur=fstart.to(fend);    
        v(verbose,format!("Figure {} took {} seconds. {}% complete. {}",i+1,dur,((i+1)*100)/n,c));
    }
    let end=PreciseTime::now();
    println!("Process ended in {} seconds.",start.to(end));

    if use_cairo{
        let mut file = File::create(output_file).unwrap();
        state.surface.write_to_png(&mut file).expect("Cair: Error writing to png"); 
    }
    else{
        //TODO: remove this ew... fuchila... guacala!!!!!
        let img_buf:Option<ImageBuffer<image::Rgba<u8>,&[u8]>>=ImageBuffer::from_raw(resize,resize,&(state.current.buf));
        assert!(img_buf.is_some());
        let resized_img=imageops::resize(&(img_buf.unwrap()),output_width, output_height,ftn);
        resized_img.save(output_file).expect("Error writing file");

    }
}
fn v(mode:bool,s:String){
    if mode {
        println!("{}",s);
    }
}
