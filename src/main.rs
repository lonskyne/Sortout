use rfd::FileDialog;
use std::{borrow::{Borrow, BorrowMut}, fs::DirEntry, rc::Rc};
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::io::Error;


slint::slint! {
    import {Button, VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component App inherits Window{
        in property <string> current_folder;
        in property <string> current_file;

        callback choose_folder <=> choose_folder_btn.clicked;
        callback open_folder <=> open_folder_btn.clicked;

        callback prev_file <=> previous_file_btn.clicked;
        callback next_file <=> next_file_btn.clicked;

        VerticalBox {
            Text { text : "Current folder: " + current_folder; }

            choose_folder_btn := Button { text: "Choose folder"; }

            open_folder_btn := Button { text: "Open folder"; }

            Text { text : "Current file:" + current_file; }

            HorizontalBox {
                previous_file_btn := Button { text: "<"; }

                next_file_btn := Button { text: ">"; }
            }
        }
    }
}
/* 
fn update_file(fi: i32, cf_mut: &mut Rc<RefCell<String>>, ) {
    cf_mut.replace(t)
}t*/

fn main() {
    let folder_path = Rc::new(RefCell::new(String::from("")));
    let current_file = Rc::new(RefCell::new(String::from("")));

    let app : App = App::new().unwrap();
    let weak = app.as_weak();

    let mut file_index = Rc::new(RefCell::new(0));let paths_vec: Vec<Result<DirEntry, Error>>
    let mut max_index = 0;

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
    let mut cf_copy = current_file.clone();

    app.on_open_folder( move || {
        let app : App = weak.upgrade().unwrap();
        
        let paths = fs::read_dir(Path::new::<str>(&(*<RefCell<String> as Clone>::clone(&fp_copy).into_inner()))).unwrap();

        let paths_vec: Vec<Result<DirEntry, Error>> = paths.collect();

        (*cf_copy.borrow_mut()).replace(paths_vec[0].as_ref().unwrap().file_name().into_string().unwrap().into());
        app.set_current_file(paths_vec[0].as_ref().unwrap().file_name().into_string().unwrap().into());

        //sortirati nekad paths po datumu kreacije
    }
    );

    app.on_next_file({
        let mut fi_copy: Rc<RefCell<i32>> = file_index.clone();

        move || {
            fi_copy.borrow_mut().replace_with(|&mut x| x + 1);
        }
    });


    app.on_prev_file({
        let mut fi_copy = Rc::clone(&file_index);

        move || {
            fi_copy.borrow_mut().replace_with(|&mut x| x - 1);
        }
    });

    app.run().unwrap();
}