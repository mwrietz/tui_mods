use crossterm::style::Color;
mod tui_frm;
mod tui_gen;
mod tui_inp;
mod tui_menu;

fn main() {
    let mut termstat = tui_gen::TermStat::default();

    tui_gen::splash_screen("Line One", "Line Two");

    // tui_gen::cls();
    //
    // tui_gen::print_page_header("TITLE");
    //
    // //println!("{}", tui_gen::get_prog_name());
    // termstat.line_check();
    //
    // let iv: i32 = tui_inp::get_val("Enter int value: ");
    // println!("{}", iv);
    //
    // let fv: f64 = tui_inp::get_val("Enter float value: ");
    // println!("{}", fv);
    //
    // let iv: i32 = tui_inp::get_val_default("Enter int value: ", 3);
    // println!("{}", iv);
    //
    // let fv: f64 = tui_inp::get_val_default("Enter float value: ", 43.234);
    // println!("{}", fv);
    //
    // let sv: String = tui_inp::get_val("Enter string: ");
    // println!("{}", sv);
    //
    // let mfrm = tui_frm::MsgFrame {
    //     frame: tui_frm::Frame {
    //         title: "temp",
    //         title_color: Color::Blue,
    //         frame_color: Color::Yellow,
    //         x: 5,
    //         y: 25,
    //         w: 40,
    //         h: 5,
    //     },
    //     msg: vec![
    //         "line one",
    //         "line two",
    //         "line three",
    //         "line four",
    //         "line five",
    //     ],
    // };
    // mfrm.frame.display();
    //
    // mfrm.frame.clear();
    // mfrm.display_msg();
    // tui_gen::pause();
    //
    // tui_gen::cls();
    // tui_gen::print_page_header("TITLE");
    //
    // // menu
    // let menu_items = vec![
    //     ("a", "Add"),
    //     ("r", "Remove"),
    //     ("e", "Edit"),
    //     ("d", "Details"),
    //     ("s", "Summary"),
    //     ("m", "Menu"),
    //     ("q", "Quit"),
    // ];
    //
    // let val = tui_menu::menu_horiz_neo(&menu_items);
    // tui_gen::cls();
    // println!("{}", val);

    /*
    let q = tui_inp::get_float_default("test", 32.234);
    termstat.line_check();
    println!("q = {}", q);
    termstat.line_check();
    let f = tui_inp::get_float_default("enter float: ", 10.2);
    termstat.line_check();
    println!("f = {}", f);
    termstat.line_check();

    let _mystr = tui_inp::dialog_box_get_string(40, 5, "String Test", "Enter String: ");

    let menu_items = vec![
        "Item one",
        "Item two",
        "Item three",
        "Item four",
        "Item five",
    ];
    tui_menu::menu("Test Inputs", &menu_items);

    let i = tui_inp::get_int("Enter int: ");
    println!("{}", i);

    let mut mfrm = tui_frm::MsgFrame {
        frame: tui_frm::Frame {
            title: "temp",
            title_color: "blue",
            frame_color: "yellow",
            x: 5,
            y: 25,
            w: 40,
            h: 5,
        },
        msg: vec![
            "line one",
            "line two",
            "line three",
            "line four",
            "line five",
        ],
    };
    mfrm.frame.display();

    mfrm.frame.clear();
    mfrm.display_msg();
    tui_gen::pause();

    mfrm.msg.push("line six");
    mfrm.frame.clear();
    mfrm.display_msg();
    tui_gen::pause();

    mfrm.msg.push("line seven");
    mfrm.frame.clear();
    mfrm.display_msg();
    tui_gen::pause();

    tui_gen::cmove(0, 20);
    println!("{:?}", mfrm.msg);

    mfrm.msg.remove(0);
    println!("{:?}", mfrm.msg);

    mfrm.msg.remove(0);
    mfrm.msg.remove(0);
    mfrm.msg.remove(0);

    mfrm.frame.clear();
    mfrm.display_msg();
    tui_gen::cmove(0, 22);

    // menu
    tui_gen::cls();
    let menu_items = vec![
        ("a", "Add"),
        ("r", "Remove"),
        ("e", "Edit"),
        ("d", "Details"),
        ("s", "Summary"),
        ("m", "Menu"),
        ("q", "Quit")
    ];

    let val = tui_menu::menu_horiz(&menu_items);
    tui_gen::cls();
    println!("{}", val);


    */
}
