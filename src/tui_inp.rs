#![allow(dead_code)]

use colored::Colorize;
use std::io::Write;

use crate::tui_gen::cmove;
use crate::tui_gen::tsize;

use crate::tui_frm::Frame;

pub fn dialog_box_get_string(width: usize, height: usize, title: &str, prompt: &str) -> String {
    let (term_width, term_height) = tsize();
    let x = (term_width - width) / 2;
    let y = (term_height - height) / 2;

    let frm = Frame {
        title,
        title_color: "white",
        frame_color: "white",
        x,
        y,
        w: width,
        h: height,
    };
    frm.display();

    // print title and get string
    cmove(x + 2, y);
    print!(" {} ", title.red());
    cmove(x + 3, y + 2);
    let s = get_string(prompt);

    s
}

pub fn get_int(prompt: &str) -> i32 {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer: i32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

pub fn get_float(prompt: &str) -> f64 {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer: f64 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

pub fn get_float_default(prompt: &str, default: f64) -> f64 {
    loop {
        let mut buffer = String::new();
        print!("{} [{:.3}]: ", prompt, default);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        // this if statment is not working
        if buffer.eq("\n") {
            return default;
        }

        let buffer: f64 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return buffer;
    }
}

pub fn get_string(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }
    return buffer;
}

pub fn get_string_default(prompt: &str, default: &str) -> String {
    let mut buffer = String::new();
    print!("{} [{}]: ", prompt, default);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }

    if buffer.eq("") {
        return default.to_string();
    } else {
        return buffer;
    }
}
