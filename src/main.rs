use rfd::FileDialog;
use slint::{ComponentHandle, Image};
use std::{borrow::BorrowMut, fs::DirEntry, rc::Rc};
use std::cell::RefCell;
use std::fs::{self, read_to_string};
use std::path::Path;

use std::io::Error;slint::slint! {
    import {Button, VerticalBox, HorizontalBox, ScrollView } from "std-widgets.slint";

    export component App inherits Window {

        in property <string> current_folder;
        in property <string> current_file;
        in property <string> current_file_type;

        in property <string> current_file_content_text;
        in property <image> current_file_content_image;

        callback choose_folder <=> choose_folder_btn.clicked;
        callback open_folder <=> open_folder_btn.clicked;

        callback prev_file <=> previous_file_btn.clicked;
        callback next_file <=> next_file_btn.clicked;

        VerticalBox {
            Text { text : "Current folder: " + current_folder; }

            choose_folder_btn := Button { text: "Choose folder"; }

            open_folder_btn := Button { text: "Open folder"; }

            VerticalBox {
                Text { text : "Current file"; }
                Text { text : "File name: " + current_file; }
                Text { text: "File type: " + current_file_type; }
                Text { text: "File contents: "; }
                Text { text: current_file_content_text;}
                Image { source: current_file_content_image; width: 60%; height: 60%; image-rendering: pixelated;}
            }

            HorizontalBox {
                previous_file_btn := Button { text: "<"; }

                next_file_btn := Button { text: ">"; }
            }
        }
    }
}

fn check_file_type(file_ext : &str) -> &'static str{
    let text_file_extensions = ["txt", "csv", "json", "c", "cpp", "py", "rs", "html", "css"];
    let image_file_extensions = ["png", "jpg", "jpeg"];

    if text_file_extensions.contains(&file_ext) { return "text"; }

    if image_file_extensions.contains(&file_ext) { return "image" };
    
    return "other";
}

fn set_current_file(dir_entry : &DirEntry, cf_ref : &mut Rc<RefCell<String>>, app_ref : &App) {
        let app = (*app_ref).clone_strong();
        let val = dir_entry.file_name().into_string().unwrap();

        cf_ref.replace(val.clone());
        app.set_current_file(val.clone().into());

        let mut file_type = String::new();
        
        match dir_entry.path().extension() {
            Some(s) => { file_type = String::from(s.to_str().unwrap()); app.set_current_file_type(file_type.clone().into()); },
            None => app.set_current_file_type(String::from("Unknown").into()),
        };

        if dir_entry.file_type().unwrap().is_dir() {
            app.set_current_file_type(String::from("Folder").into());
        }

        if dir_entry.file_type().unwrap().is_symlink() {
            app.set_current_file_type(String::from("Symlink").into());
        }

        
        app.set_current_file_content_text("".into());
        app.set_current_file_content_image(Image::default());

        match check_file_type(file_type.as_str()) {
            "text" => app.set_current_file_content_text(read_to_string(dir_entry.path()).unwrap_or(String::from("Text file read error!"))[0..255].into()),
            "image" => app.set_current_file_content_image(Image::load_from_path(dir_entry.path().as_path()).unwrap_or(Image::load_from_path(Path::new("./resources/err_img.png")).unwrap())),
            "other" => app.set_current_file_content_text(String::from("Cannot read this file type.").into()),
            _ => app.set_current_file_content_text(String::from("Error when matching file types!").into())
        }
}

fn main() {
    let folder_path = Rc::new(RefCell::new(String::from("")));
    let current_file = Rc::new(RefCell::new(String::from("")));

    let app : App = App::new().unwrap();
    let weak = app.as_weak();

    let file_index = Rc::new(RefCell::new(0));
    let paths_vec: Rc<RefCell<Vec<Result<DirEntry, Error>>>> = Rc::new(RefCell::new(Vec::new()));
    let max_index = Rc::new(RefCell::new(0));


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
    let pv_copy = paths_vec.clone();
    let maxind_copy = max_index.clone();

    app.on_open_folder( {
        let app : App = weak.upgrade().unwrap();

        move ||  {
            let paths_wrapped = fs::read_dir(Path::new::<str>(&(*<RefCell<String> as Clone>::clone(&fp_copy).into_inner())));

            match paths_wrapped {
                Ok(f) => { pv_copy.replace(f.collect()); (); },
                Err(e) => println!("{:?}", e),
            };

            if pv_copy.borrow().len() > 0 {
                set_current_file(pv_copy.borrow()[0].as_ref().unwrap(), cf_copy.borrow_mut(), &app);
            }

            maxind_copy.replace(pv_copy.borrow().len());
            //sortirati nekad paths po datumu kreacije
        }
    });


    app.on_next_file({
        let app : App = weak.upgrade().unwrap();

        let mut cf_copy = current_file.clone();
        let mut fi_copy: Rc<RefCell<i32>> = file_index.clone();
        let pv_copy = paths_vec.clone();

        move || {
            if *fi_copy.borrow()+1 < (*max_index.borrow()).try_into().unwrap() {
                fi_copy.borrow_mut().replace_with(|&mut x| x + 1);

                let new_val = *fi_copy.borrow();

                set_current_file(pv_copy.borrow()[new_val as usize].as_ref().unwrap(), cf_copy.borrow_mut(), &app);
            }
        }
    });


    app.on_prev_file({
        let app : App = weak.upgrade().unwrap();

        let mut cf_copy = current_file.clone();
        let mut fi_copy: Rc<RefCell<i32>> = file_index.clone();
        let pv_copy = paths_vec.clone();

        move || {
            if *fi_copy.borrow() > 0 {
                fi_copy.borrow_mut().replace_with(|&mut x| x - 1);

                let new_val = *fi_copy.borrow();

                set_current_file(pv_copy.borrow()[new_val as usize].as_ref().unwrap(), cf_copy.borrow_mut(), &app); 
            }
        }
    });

    
    app.run().unwrap();
}