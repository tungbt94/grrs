use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use grrs::find_matches;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure>{
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|p| format!("Could not read file `{}`", p))?;

    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    // let stdout = io::stdout();
    // let mut handle = stdout.lock();
    // writeln!(handle, "foo: {}", 42);
    find_matches(&content, &args.pattern, & mut std::io::stdout());
    Ok(())
}
