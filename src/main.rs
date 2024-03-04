use rfd::FileDialog;
use std::{borrow::{Borrow, BorrowMut}, rc::Rc};
use std::cell::RefCell;
use std::fs;
use std::path::Path;


slint::slint! {
    import {Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window{
        in property <string> current_folder;

        callback choose_folder <=> choose_folder_btn.clicked;
        callback open_folder <=> open_folder_btn.clicked;

        VerticalBox {
            Text { text : "Current folder: " + current_folder; }

            choose_folder_btn := Button { text: "Choose folder"; }

            open_folder_btn := Button { text: "Open folder"; }
        }
    }
}

fn main() {
    let folder_path = Rc::new(RefCell::new(String::from("")));

    let app : App = App::new().unwrap();
    let weak = app.as_weak();

    let mut fp_copy = folder_path.clone();

    app.on_choose_folder( {
        let app : App = weak.upgrade().unwrap();

        move || {
            let folder = FileDialog::new()
                .set_directory("/")
                .pick_folder();

            match folder { 
                Some(f) => { (*fp_copy.borrow_mut()).replace(f.clone().into_os_string().into_string().unwrap()); 
                                        app.set_current_folder(f.clone().into_os_string().into_string().unwrap().into()); }, 
                None => (),
            };
        }
    }
    );

    let fp_copy = folder_path.clone();
    app.on_open_folder( move || {
        let app : App = weak.upgrade().unwrap();
        
        let paths = fs::read_dir(Path::new::<str>(&(*<RefCell<String> as Clone>::clone(&fp_copy).into_inner()))).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }
    );

    app.run().unwrap();
}