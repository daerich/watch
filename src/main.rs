use ncurses;
use std::thread;
use std::process;
use std::time::Duration;
use std::env;
use std::sync::mpsc;

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

fn main() -> std::result::Result<(),()> {
    
    if env::args().len() <= 1{
        print!("USAGE:\n watch [COMMAND]\n");
        return Ok(());
    }
    let mut arg_str = String::new(); /* Create prompt string */
    for arg in env::args(){
        arg_str.push_str(" ");
        arg_str.push_str(&arg);
    }
    
    let shell = get_shell();
    /* Construct command string */
    let mut command_str = String::new();
    {
        let mut count = 0;
        for arg in env::args(){
           
            if count == 0{
                count += 1;
                continue;
            }
            command_str.push_str(" ");
            command_str.push_str(&arg);
            count += 1;
        }
    }
    /* Create tui */
    ncurses::initscr();
    /* Start output routine */
    loop{
        ncurses::clear();
        let mut strn = String::new();
        ncurses::mvaddstr(0, 0, &arg_str);
        ncurses::mvaddstr(1, 0, "---");
        let resstr = run_command(&shell, &command_str);
        strn.push_str(&resstr);
        ncurses::mvaddstr(2, 0,&strn);
        ncurses::refresh();
        thread::sleep(Duration::new(2, 0));
    }
    ncurses::endwin(); /* Unreachable but call it anyways */
    Ok(())
}
