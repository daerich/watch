use ncurses;
use getopts;
use std::thread;
use std::process;
use std::time::Duration;
use std::env;
use std::sync::mpsc;
use std::ops::Add;
use std::process::exit;

fn get_shell() -> std::string::String{
    for var in env::vars(){
        if var.0 == "SHELL"{
            let mut command = process::Command::new(&var.1); /* Check shell compatibilty */
            command.arg("-i");
            command.arg("-c");
            command.arg(" ");
            if command.status().unwrap().success(){
            return var.1;
            }
            else {
                return String::from("sh"); /* Choose POSIX if check fails*/
            }
        }
    }
    String::from("sh") /* Choose POSIX version */
}

fn run_command(shell : &std::string::String, arg_str: &std::string::String) -> std::string::String{
    let mut command = process::Command::new(shell);
    command.arg("-i");
    command.arg("-c");
    command.arg(arg_str);
    let (sn,rec) = mpsc::channel();
    std::thread::spawn(move || {
        let res = String::from_utf8(command.output().unwrap().stdout).unwrap();
        sn.send(res).unwrap();   
    });
    let res = rec.recv().unwrap();
    res
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

    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.opt("n", "interval", "Set refresh interval", "Integer", getopts::HasArg::Yes, getopts::Occur::Optional);
    let optstring : Vec<String> = env::args().collect();
    
    if env::args().len() <= 1{
        let progstr = env::args().nth(0).unwrap() + " [OPTION]" + " [COMMAND]";
        println!("{}",opts.usage(&progstr));
        return Ok(());
    }
    /* Construct program parameters */
    let durstr : String;
    let mut command_str = String::new();

    if let Ok(res) = opts.parse(&optstring[1..]){ /* argparse */
        if res.opt_present("n") {
            durstr = res.opt_str("n").unwrap();
        }
        else{
            durstr = String::from("2");
        }

        let sanstr = res.free.clone(); /* Concat */
        for frag in sanstr{
            command_str.push_str(&frag);
            command_str.push_str(" ");
        }
    }
    else{
        println!("Couldn't parse optstring!\nMaybe you used illegal arguments?\
        \nOr too many of them, or multiple times,or..Computer says no?");
        exit(1); /* Better than a ugly panic */
    };
    
    let mut arg_str = String::new(); /* Create prompt string */
    for arg in env::args(){
        arg_str.push_str(" ");
        arg_str.push_str(&arg);
    }
    
    let shell = get_shell();
    let duration = Duration::new(atoi(&durstr),0);

    /* Create tui */
    ncurses::initscr();
    let timestr =  String::from("Every ").add(&durstr).add(" seconds:");
    /* Start output routine */
    loop{
        ncurses::clear();
        let mut strn = String::new();
        ncurses::mvaddstr(0, 0, &timestr);
        ncurses::mvaddstr(1, 0, &arg_str);
        ncurses::mvaddstr(2, 0, "---");
        let resstr = run_command(&shell, &command_str);
        strn.push_str(&resstr);
        ncurses::mvaddstr(3, 0,&strn);
        ncurses::refresh();
        thread::sleep(duration);
    }
    ncurses::endwin(); /* Unreachable but call it anyways */ 
    Ok(())
}
