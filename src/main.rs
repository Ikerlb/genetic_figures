mod state;
mod imgutil;
mod rectangle;
mod util;
mod figure;
mod minimize;
mod ellipse;
mod ga;

use image::{imageops,FilterType};
use time::PreciseTime;
use clap::{Arg, App};
use state::State;
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
                    .arg(Arg::with_name("resize")
                        .short("r")
                        .long("resize")
                        .required(false)
                        .help("resize target image to this value")
                        .takes_value(true))
                    .arg(Arg::with_name("outputSize")
                        .short("s")
                        .long("size")
                        .required(false)
                        .help("size of output image")
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
                .get_matches();

    let input_file=matches.value_of("inputFile").unwrap();
    let output_file=matches.value_of("outputFile").unwrap();
    let alpha:u8=matches.value_of("alpha")
                        .unwrap_or("127")
                        .parse::<u8>()
                        .unwrap();
    let resize:u32=matches.value_of("resize")
                        .unwrap_or("256")
                        .parse::<u32>()
                        .unwrap();
    let n=matches.value_of("numberOfFigures")
                 .unwrap()
                 .parse::<u32>()
                 .unwrap();
    let output_size=matches.value_of("size")
                           .unwrap_or("1024")
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
    let verbose=matches.is_present("verbose");
    let t=image::open(input_file).unwrap().to_rgba();
    let target=imageops::resize(&t,resize,resize,FilterType::Nearest);
    let mut state=State::new(target,alpha);
    println!("Starting...");
    let start=PreciseTime::now();
    for i in 0..n{
        let fstart=PreciseTime::now();
        state.step(g,p,mode);
        let fend=PreciseTime::now();
        v(verbose,format!("Figure {} took {} seconds. {}% complete.",i+1,fstart.to(fend),((i+1)*100)/n));
    }
    let end=PreciseTime::now();
    println!("Process ended in {} seconds.",start.to(end));
    let img=imageops::resize(&state.current,output_size,output_size,FilterType::Nearest);
    img.save(output_file).unwrap();
}

fn v(mode:bool,s:String){
    if mode {
        println!("{}",s);
    }
}
