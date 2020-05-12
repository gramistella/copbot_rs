extern crate gtk;

use gtk::prelude::*;

#[derive(Clone)]
pub struct LogHandler{
    pub txt_buffer: gtk::TextBuffer,
    pub debug: bool
}

impl LogHandler{
    pub fn print_to_log(&self, line: &str) {
        if self.debug{
            let mut end_iter = self.txt_buffer.get_end_iter();
            let to_append = format!("{}\n", line);
            &self.txt_buffer.insert(&mut end_iter, &to_append);
        }
    }
    // pub fn println(&self, line: &str){
    //     if self.debug{
    //         println!("{}", line);
    //     }
    // }
}