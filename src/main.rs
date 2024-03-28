use rfd::FileDialog;
use std::{borrow::BorrowMut, fs::DirEntry, rc::Rc};
use std::cell::RefCell;
use std::fs::{self, read_to_string};
use std::path::Path;
use trash;
use std::io::Error;

use slint::Image;
slint::include_modules!();

fn check_file_type(file_ext : &str) -> &'static str{
    let text_file_extensions = ["txt", "csv", "json", "c", "cpp", "py", "rs", "html", "css"];
    let image_file_extensions = ["png", "jpg", "jpeg"];

    if text_file_extensions.contains(&file_ext.to_lowercase().as_str()) { return "text"; }

    if image_file_extensions.contains(&file_ext.to_lowercase().as_str()) { return "image"; }
    
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

fn clear_app_ui(app_ref : &App) {
    let app = (*app_ref).clone_strong();

    app.set_folder_opened(false);
    app.set_folder_chosen(false);

    app.set_current_file(String::from("").into());
    app.set_current_file_content_text(String::from("").into());
    app.set_current_file_type(String::from("").into());
    app.set_current_folder(String::from("").into());
    app.set_marked_deletion_list(String::from("").into());
    app.set_current_file_content_image(Image::default());
}

fn main() -> Result<(), slint::PlatformError> {
    let folder_path = Rc::new(RefCell::new(String::from("")));
    let current_file = Rc::new(RefCell::new(String::from("")));

    let marked_deletion = Rc::new(RefCell::new(Vec::<i32>::new()));

    let app = App::new()?;
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
                                        app.set_current_folder(f.clone().into_os_string().into_string().unwrap().into()); 
                                        app.set_folder_chosen(true);
                                    }, 
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

            app.set_folder_opened(true);
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


    app.on_mark_delete({
        let app : App = weak.upgrade().unwrap();

        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();

        let mark_del_copy = marked_deletion.clone();

        move || {
            let mut mark_del_ref = mark_del_copy.try_borrow_mut().unwrap();

            if !mark_del_ref.contains(&*fi_copy.borrow()) {
                (*mark_del_ref).push(*fi_copy.borrow());

                let val = pv_copy.borrow()[*fi_copy.borrow() as usize].as_ref().unwrap().file_name().into_string().unwrap();
                let mut string = String::from(app.get_marked_deletion_list().as_str());

                string.push_str(val.as_str());
                string.push('\n');

                app.set_marked_deletion_list(string.into());
            }
        }
    });


    app.on_confirm_marks({
        let app : App = weak.upgrade().unwrap();
        let mark_del_copy = marked_deletion.clone();
        let pv_copy = paths_vec.clone();

        move || {
            let mark_del_ref = mark_del_copy.borrow();


            for i in mark_del_ref.iter() {
                let path = pv_copy.borrow()[*i as usize].as_ref().unwrap().path();

                match trash::delete(path) {
                    Ok(_e) => (),
                    Err(e) => println!("{:?}", e),
                };
            }

            app.set_marked_deletion_list(String::from("").into());
            
            let mark_del_refmut = mark_del_copy.try_borrow_mut();
            match mark_del_refmut {
                Ok(mut f) => { f.clear(); () },
                Err(_) => (),
            };

            clear_app_ui(&app);
        }
    });
    
    app.run()
}