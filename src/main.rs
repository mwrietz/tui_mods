mod tui_frm;
mod tui_gen;
mod tui_inp;
mod tui_menu;

fn main() {
    tui_gen::cls();

    let q = tui_inp::get_float_default("test", 5.4);
    println!("q = {}", q);
    let f = tui_inp::get_float_default("enter float: ", 10.2);
    println!("f = {}", f);

    let _mystr = tui_inp::dialog_box_get_string(40, 5, "String Test", "Enter String: ");

    let menu_items = vec!["Item one", "Item two", "Item three"];
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
    tui_gen::cmove(0,22);
}
