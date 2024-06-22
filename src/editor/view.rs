const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");

use super::terminal::Terminal;
pub struct View {

}

impl View {
    
    fn draw_empty_row()->Result<(),std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_line_feed()->Result<(),std::io::Error> {
        Terminal::print("\r\n")?;
        Ok(())
    }
    fn draw_welcome_info()->Result<(),std::io::Error> {
        let mut msg=format!("{NAME} editor -- version {VERSION}");
        let width=Terminal::terminal_size()?.width;
        let len=msg.len();
        // Terminal::print(format!("{len} {width}\r\n"))?;
        let padding_num=(width.saturating_sub(len)).saturating_div(2);
        let padding=" ".repeat(padding_num-1);
        msg=format!("~{padding}{msg}");
        msg.truncate(width);
        Terminal::print(msg)?;
        Ok(())
    }
    pub fn render()->Result<(),std::io::Error>{
        let height=Terminal::terminal_size()?.height;
        for row_num in 0..height-1{
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if row_num==1 {
                Terminal::print("Hello World!\r\n")?;
            } else if row_num==height/3 {
                Self::draw_welcome_info()?;
            } else {
                Self::draw_empty_row()?;
            }
            Self::draw_line_feed()?;
        }
        Self::draw_empty_row()?;
        Ok(())

    }
}