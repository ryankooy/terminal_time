use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;

fn main() -> io::Result<()> {
    let mut st = stdout().into_raw_mode().unwrap();
    write!(
        st,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Hide
    ).unwrap();

    let mut mv1: u16 = 1;
    let mut mv2: u16 = 1;
    let mut ct: u16 = 0;

    while ct < 60 {
        let thing: &str =
            if ct % 5 == 0 {
                "o"
            } else if ct % 3 == 0 {
                "-"
            } else {
                "."
            };

        writeln!(st, "{}", thing)?;

        thread::sleep(Duration::from_millis(20));
        write!(
            st,
            "{}{}",
            termion::cursor::Goto(mv1, mv2),
            termion::clear::All
        ).unwrap();
        st.flush().unwrap();

        mv1 += 1;
        if ct < 30 { mv2 += 1; } else { mv2 -= 1; }
        ct += 1;
    }

    thread::sleep(Duration::from_millis(500));
    write!(
        st,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Show
    ).unwrap();

    Ok(())
}
