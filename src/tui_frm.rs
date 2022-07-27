use colored::Colorize;
use crate::tui_gen::cmove;

pub struct Frame {
    pub title: String,
    pub title_color: String,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Frame {
    #[allow(dead_code)]
    pub fn clear(&self) {
        // draw middle
        for i in 0..(self.h-1) {
            cmove(self.x+1, self.y+i+1);
            for _j in 0..(self.w-2) {
                print!(" ");
            }
        }
    }
    pub fn display(&self) {
        let ul = "╭".white();
        let ur = "╮".white();
        let ll = "╰".white();
        let lr = "╯".white();
        let hor = "─".white();
        let ver = "│".white();

        // draw top horizontal
        cmove(self.x, self.y);
        print!("{}", ul);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        print!("{}", ur);

        // draw middle
        for i in 0..(self.h-1) {
            cmove(self.x, self.y+i+1);
            print!("{}", ver);
            for _j in 0..(self.w-2) {
                print!(" ");
            }
            print!("{}", ver);
        }

        // draw bottom horizontal
        cmove(self.x, self.y+self.h);
        print!("{}", ll);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        println!("{}", lr);

        if self.title.len() > 0 {
            // print title 
            cmove(self.x+2, self.y);
            if self.title_color == "red" {
                print!(" {} ", self.title.red());
            }
            if self.title_color == "green" {
                print!(" {} ", self.title.green());
            }
            if self.title_color == "blue" {
                print!(" {} ", self.title.blue());
            }
            if self.title_color == "yellow" {
                print!(" {} ", self.title.yellow());
            }
            if self.title_color == "purple" {
                print!(" {} ", self.title.purple());
            }
            if self.title_color == "white" {
                print!(" {} ", self.title.white());
            }
        }
    }
}

pub struct MsgFrame {
    pub frame: Frame,
    pub msg: Vec<String>,
}

impl MsgFrame {
    pub fn display_msg(&self) {
        for i in 0..self.msg.len() {
            if self.msg.len() > (self.frame.h - 1) {
                if i > (self.msg.len() - self.frame.h) {
                    cmove(self.frame.x + 2, self.frame.y + (i - (self.msg.len() - self.frame.h)));
                    print!("{}", self.msg[i]);
                }
            } else {
                cmove(self.frame.x + 2, self.frame.y + (i + 1));
                print!("{}", self.msg[i]);
            }
        }
    }
}

