use ncurses;
use getopts;
use std::thread;
use std::process;
use std::time::Duration;
use std::env;
use std::sync::mpsc;
use std::ops::Add;
use std::process::exit;

fn is_shell(shell:&str) -> bool{
    let mut command = process::Command::new(shell); /* Check shell compatibilty */
    command.arg("-i");
    command.arg("-c");
    command.arg(" ");
    if let Ok(_) = command.status(){
        return true;
    }else{
        return false;
    }
}

fn get_shell(sover:&str) -> String{

    if !sover.is_empty(){ /* Check overrides */
        if is_shell(sover){
            return String::from(sover);
        }
    }
    for var in env::vars(){
        if var.0 == "SHELL"{ /* If not get primary/login shell */
            if is_shell(&var.1){
                return String::from(&var.1);
            }
            else {
                return String::from("sh"); /* Choose POSIX if check fails*/
            }
        }
    }
    String::from("sh") /* Choose POSIX version */
}

fn run_command(shell : &String, arg_str: &String,duration:Duration, is_force:bool) -> mpsc::Receiver<String>{
    let mut command = process::Command::new(shell);
    command.arg("-i");
    command.arg("-c");
    command.arg(arg_str);
    let (sn,rec) = mpsc::channel();
    std::thread::spawn(move || {
        loop{
        let res = if let Ok(value) = String::from_utf8(
            if let Ok(value) = command.output(){ /* Stop thread on error (looks ugly) */
                if value.status.success()|| is_force{
                     value.stdout
                }
                else{
                     value.stderr
                }
            }
            else{
                break;
            }){
              value  
            }
            else{
                break;
            };
        sn.send(res).unwrap();
        thread::sleep(duration);
        } 
    });
    rec
}

fn atoi(dur : &String) -> u64{
    match dur.parse::<u64>(){
        Ok(res) => {
            res
        }
        Err(_) =>{
            println!("Please specify an integer");
            exit(1);
        }
    }
}

fn main() -> std::result::Result<(),()> {
    /* VERSION: */
    const VERSION : &str = "1.7";

    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.opt("n", "interval", "Set refresh interval", "Integer", getopts::HasArg::Yes, getopts::Occur::Optional);
    opts.opt("s", "shell", "Override shell", "String", getopts::HasArg::Yes, getopts::Occur::Optional);
    opts.opt("", "force-stdout", "Force STDOUT", "String", getopts::HasArg::No, getopts::Occur::Optional);
    opts.opt("v", "version", "Print version and exit", "", getopts::HasArg::No, getopts::Occur::Optional);
    let optstring : Vec<String> = env::args().collect();
    
    if env::args().len() <= 1{
        let progstr = env::args().nth(0).unwrap() + " [OPTION]" + " [COMMAND]";
        println!("{}",opts.usage(&progstr));
        return Ok(());
    }
    /* Construct program parameters */
    let durstr : String;
    let mut command_str = String::new();
    let mut shell_ovrr = String::new();
    let stdout_forced;

    if let Ok(res) = opts.parse(&optstring[1..]){ /* argparse */

        if res.opt_present("v"){
            println!("hawk/watch\nVersion: {}\nBy DaErich",VERSION);
            exit(0);
            /* not reached */
        }
        if res.opt_present("n") {
            durstr = res.opt_str("n").unwrap();
        }
        else{
            durstr = String::from("2");
        }
        if res.opt_present("force-stdout"){
            stdout_forced = true;
        }
        else{
            stdout_forced = false;
        }
        if res.opt_present("s"){
            shell_ovrr.push_str(&res.opt_str("s").unwrap());
        }
        let sanstr = res.free.clone(); /* Concat */
        for frag in sanstr{
            command_str.push_str(&frag);
            command_str.push_str(" ");
        }
    }
    else{
        println!("Couldn't parse Options!\nMaybe you used illegal arguments?\
        \nOr too many of them, or multiple times,or..Computer says no?");
        exit(1); /* Better than a ugly panic */
    };
    
    let mut arg_str = String::new(); /* Create prompt string */
    for arg in env::args(){
        arg_str.push_str(" ");
        arg_str.push_str(&arg);
    }
    
    let shell = get_shell(&shell_ovrr);
    let duration = Duration::new(atoi(&durstr),0);

    /* Create tui */
    ncurses::initscr();
    let timestr =  String::from("Every ").add(&duration.as_secs().to_string()).add(
        || -> &str{ if duration.as_secs() == 1 {return " second"} else {return " seconds"}  }());

    /* Start output routine */
    let mut resstr;
    let mut receiver = run_command(&shell, &command_str, duration, stdout_forced);
    loop{
        resstr=String::new();
        ncurses::clear();
        ncurses::mvaddstr(0, 0, &timestr);
        ncurses::mvaddstr(1, 0, &arg_str);
        ncurses::mvaddstr(2, 0, "---");
        if let Ok(strn) = receiver.recv(){
            resstr.push_str(&strn);
        }else{
            receiver=run_command(&shell, &command_str, duration, stdout_forced);
            resstr.push_str(&receiver.recv().expect("Cannot create new thread!Exiting!")) /* Try again, and exit if fubar  */
        }
        ncurses::mvaddstr(3, 0,&resstr);
        ncurses::refresh();
        thread::sleep(duration);
    }
    ncurses::endwin(); /* Unreachable but call it anyways */
    Ok(())
}
