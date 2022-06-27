pub struct Window {
    replaced_region: (),
}

impl Window {
    pub fn new(rows: u16, cols: u16) -> Self {
        // copy replaced_region
        let (full_cols, full_rows) = crossterm::terminal::size().unwrap();
        
        let parser = vt100::Parser::new(full_rows, full_cols, 0);
        let screen = parser.screen();
        
        let contents = screen.contents_between(0, 0, full_rows, 0);
        
        println!("{:?}", contents);
        
        Window { replaced_region: () }
    }
}

impl std::io::Write for Window {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // redraw replaced_region
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Window;
    
    #[test]
    fn it_works() {
        Window::new(0, 0);
    }
}
