use rfd::FileDialog;
use std::{borrow::BorrowMut, fs::DirEntry, rc::Rc};
use std::cell::{Ref, RefCell};
use std::fs::{self, read_to_string, rename};
use std::path::{Path, PathBuf};
use trash;
use std::io::Error;
use array_init::array_init;

use slint::Image;
slint::include_modules!();

const MAX_FOLDER_NUM: usize = 12;

fn check_file_type(file_ext : &str) -> &'static str{
    let text_file_extensions = ["txt", "csv", "json", "c", "cpp", "py", "rs", "html", "css"];
    let image_file_extensions = ["png", "jpg", "jpeg"];

    if text_file_extensions.contains(&file_ext.to_lowercase().as_str()) { return "text"; }

    if image_file_extensions.contains(&file_ext.to_lowercase().as_str()) { return "image"; }
    
    return "other";
}

fn set_current_color(app_ref : &App) {
    let app = (*app_ref).clone_strong();

    if app.get_marked_deletion_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_del_color()); }
    else if app.get_marked_f1_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f1_color()); }
    else if app.get_marked_f2_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f2_color()); }
    else if app.get_marked_f3_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f3_color()); }
    else if app.get_marked_f4_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f4_color()); }
    else if app.get_marked_f5_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f5_color()); }
    else if app.get_marked_f6_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f6_color()); }
    else if app.get_marked_f7_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f7_color()); }
    else if app.get_marked_f8_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f8_color()); }
    else if app.get_marked_f9_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f9_color()); }
    else if app.get_marked_f10_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f10_color()); }
    else if app.get_marked_f11_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f11_color()); }
    else if app.get_marked_f12_list().contains(app.get_current_file().as_str()) {
        app.set_cur_color(app.get_f12_color()); }
    else { app.set_cur_color(app.get_default_color()); }
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

        set_current_color(app_ref);


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
    app.set_marked_f1_list(String::from("").into());
    app.set_marked_f2_list(String::from("").into());
    app.set_marked_f3_list(String::from("").into());
    app.set_marked_f4_list(String::from("").into());
    app.set_marked_f5_list(String::from("").into());
    app.set_marked_f6_list(String::from("").into());
    app.set_marked_f7_list(String::from("").into());
    app.set_marked_f8_list(String::from("").into());
    app.set_marked_f9_list(String::from("").into());
    app.set_marked_f10_list(String::from("").into());
    app.set_marked_f11_list(String::from("").into());
    app.set_marked_f12_list(String::from("").into());
    app.set_current_file_content_image(Image::default());
}

fn set_sort_folder(sort_folder_path: &mut Rc<RefCell<String>>, sf_ind : i32, app_ref : &App) {
    let app = (*app_ref).clone_strong();

        let folder = FileDialog::new()
            .set_directory("/")
            .pick_folder();

        match folder { 
            Some(f) => { 
                sort_folder_path.replace(f.clone().into_os_string().into_string().unwrap());
                match sf_ind {
                    0 => app.set_folder1_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    1 => app.set_folder2_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    2 => app.set_folder3_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    3 => app.set_folder4_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    4 => app.set_folder5_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    5 => app.set_folder6_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    6 => app.set_folder7_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    7 => app.set_folder8_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    8 => app.set_folder9_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    9 => app.set_folder10_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    10 => app.set_folder11_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    11 => app.set_folder12_name(f.clone().file_name().unwrap().to_str().unwrap().into()),
                    _ => ()
                };    
            }, 
            None => (),
        };
}

fn mark_move(path_vec: Ref<Vec<Result<DirEntry, Error>>>,mark_move: &mut Rc<RefCell<Vec<i32>>> ,file_index: Ref<i32>,sf_ind : i32, app_ref : &App) {
    let app = (*app_ref).clone_strong();

    if !mark_move.borrow().contains(&*file_index) {
        (*mark_move).try_borrow_mut().unwrap().push(*file_index);

        let val = path_vec[*file_index as usize].as_ref().unwrap().file_name().into_string().unwrap();
        let mut string ;

        match sf_ind {
            0 => string = String::from(app.get_marked_f1_list().as_str()),
            1 => string = String::from(app.get_marked_f2_list().as_str()),
            2 => string = String::from(app.get_marked_f3_list().as_str()),
            3 => string = String::from(app.get_marked_f4_list().as_str()),
            4 => string = String::from(app.get_marked_f5_list().as_str()),
            5 => string = String::from(app.get_marked_f6_list().as_str()),
            6 => string = String::from(app.get_marked_f7_list().as_str()),
            7 => string = String::from(app.get_marked_f8_list().as_str()),
            8 => string = String::from(app.get_marked_f9_list().as_str()),
            9 => string = String::from(app.get_marked_f10_list().as_str()),
            10 => string = String::from(app.get_marked_f11_list().as_str()),
            11 => string = String::from(app.get_marked_f12_list().as_str()),
            _ => string = String::new()
        }

        string.push_str(val.as_str());
        string.push('\n');

        match sf_ind {
            0 => app.set_marked_f1_list(string.into()),
            1 => app.set_marked_f2_list(string.into()),
            2 => app.set_marked_f3_list(string.into()),
            3 => app.set_marked_f4_list(string.into()),
            4 => app.set_marked_f5_list(string.into()),
            5 => app.set_marked_f6_list(string.into()),
            6 => app.set_marked_f7_list(string.into()),
            7 => app.set_marked_f8_list(string.into()),
            8 => app.set_marked_f9_list(string.into()),
            9 => app.set_marked_f10_list(string.into()),
            10 => app.set_marked_f11_list(string.into()),
            11 => app.set_marked_f12_list(string.into()),
            _ => ()
        }
        
    }

    set_current_color(app_ref);
}

fn move_to_folder(mark_f_ref: Ref<Vec<i32>>, pv_copy: Ref<Vec<Result<DirEntry, Error>>>, folder_path: Ref<String>) {
    for i in mark_f_ref.iter() {
        let path = pv_copy[*i as usize].as_ref().unwrap().path();

        match rename(path.clone(), Path::new(Path::new::<str>(&((String::from(folder_path.as_str()) + "/" + path.file_name().unwrap().to_str().unwrap()))))) {                    
            Ok(_a) => (),
            Err(e) => { print!("{:?}", e); }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let folder_path = Rc::new(RefCell::new(String::from("")));
    let current_file = Rc::new(RefCell::new(String::from("")));

    let move_folder_paths: Rc<[Rc<RefCell<String>>; MAX_FOLDER_NUM]> = Rc::new(array_init(|_| Rc::new(RefCell::new(String::new()))));

    let marked_deletion = Rc::new(RefCell::new(Vec::<i32>::new()));
    let marked_move_arr: [Rc<RefCell<Vec<i32>>>; MAX_FOLDER_NUM] = array_init(|_| Rc::new(RefCell::new(vec![])));

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

            set_current_color(&app);
        }
    });


    app.on_confirm_marks({
        let app : App = weak.upgrade().unwrap();

        let mark_del_copy = marked_deletion.clone();
        let mark_f1_copy = marked_move_arr[0].clone();
        let mark_f2_copy = marked_move_arr[1].clone();
        let mark_f3_copy = marked_move_arr[2].clone();
        let mark_f4_copy = marked_move_arr[3].clone();
        let mark_f5_copy = marked_move_arr[4].clone();
        let mark_f6_copy = marked_move_arr[5].clone();
        let mark_f7_copy = marked_move_arr[6].clone();
        let mark_f8_copy = marked_move_arr[7].clone();
        let mark_f9_copy = marked_move_arr[8].clone();
        let mark_f10_copy = marked_move_arr[9].clone();
        let mark_f11_copy = marked_move_arr[10].clone();
        let mark_f12_copy = marked_move_arr[11].clone();

        let folder1_path_copy = move_folder_paths[0].clone();
        let folder2_path_copy = move_folder_paths[1].clone();
        let folder3_path_copy = move_folder_paths[2].clone();
        let folder4_path_copy = move_folder_paths[3].clone();
        let folder5_path_copy = move_folder_paths[4].clone();
        let folder6_path_copy = move_folder_paths[5].clone();
        let folder7_path_copy = move_folder_paths[6].clone();
        let folder8_path_copy = move_folder_paths[7].clone();
        let folder9_path_copy = move_folder_paths[8].clone();
        let folder10_path_copy = move_folder_paths[9].clone();
        let folder11_path_copy = move_folder_paths[10].clone();
        let folder12_path_copy = move_folder_paths[11].clone();


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

            move_to_folder(mark_f1_copy.borrow(), pv_copy.borrow(), folder1_path_copy.borrow());
            move_to_folder(mark_f2_copy.borrow(), pv_copy.borrow(), folder2_path_copy.borrow());
            move_to_folder(mark_f3_copy.borrow(), pv_copy.borrow(), folder3_path_copy.borrow());
            move_to_folder(mark_f4_copy.borrow(), pv_copy.borrow(), folder4_path_copy.borrow());
            move_to_folder(mark_f5_copy.borrow(), pv_copy.borrow(), folder5_path_copy.borrow());
            move_to_folder(mark_f6_copy.borrow(), pv_copy.borrow(), folder6_path_copy.borrow());
            move_to_folder(mark_f7_copy.borrow(), pv_copy.borrow(), folder7_path_copy.borrow());
            move_to_folder(mark_f8_copy.borrow(), pv_copy.borrow(), folder8_path_copy.borrow());
            move_to_folder(mark_f9_copy.borrow(), pv_copy.borrow(), folder9_path_copy.borrow());
            move_to_folder(mark_f10_copy.borrow(), pv_copy.borrow(), folder10_path_copy.borrow());
            move_to_folder(mark_f11_copy.borrow(), pv_copy.borrow(), folder11_path_copy.borrow());
            move_to_folder(mark_f12_copy.borrow(), pv_copy.borrow(), folder12_path_copy.borrow());

            let mark_del_refmut = mark_del_copy.try_borrow_mut();
            match mark_del_refmut {
                Ok(mut f) => { f.clear(); () },
                Err(_) => (),
            };

            clear_app_ui(&app);
        }
    });

    app.on_choose_sort_folder1({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[0].clone().borrow_mut(), 0, &app); }
    });

    app.on_choose_sort_folder2({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[1].clone().borrow_mut(), 1, &app); }
    });

    app.on_choose_sort_folder3({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[2].clone().borrow_mut(), 2, &app); }
    });

    app.on_choose_sort_folder4({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[3].clone().borrow_mut(), 3, &app); }
    });

    app.on_choose_sort_folder5({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[4].clone().borrow_mut(), 4, &app); }
    });

    app.on_choose_sort_folder6({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[5].clone().borrow_mut(), 5, &app); }
    });

    app.on_choose_sort_folder7({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[6].clone().borrow_mut(), 6, &app); }
    });

    app.on_choose_sort_folder8({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[7].clone().borrow_mut(), 7, &app); }
    });

    app.on_choose_sort_folder9({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[8].clone().borrow_mut(), 8, &app); }
    });

    app.on_choose_sort_folder10({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[9].clone().borrow_mut(), 9, &app); }
    });

    app.on_choose_sort_folder11({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[10].clone().borrow_mut(), 10, &app); }
    });

    app.on_choose_sort_folder12({
        let app = weak.upgrade().unwrap();
        let mfp_copy = move_folder_paths.clone();

        move || { set_sort_folder(mfp_copy[11].clone().borrow_mut(), 11, &app); }
    });

    app.on_mark_move_f1({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[0].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 0, &app)  }
    });

    app.on_mark_move_f2({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[1].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 1, &app) }
    });

    app.on_mark_move_f3({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[2].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 2, &app) }
    });
    
    app.on_mark_move_f4({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[3].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 3, &app) }
    });

    app.on_mark_move_f5({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[4].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 4, &app) }
    });

    app.on_mark_move_f6({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[5].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 5, &app) }
    });

    app.on_mark_move_f7({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[6].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 6, &app) }
    });
    
    app.on_mark_move_f8({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[7].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 7, &app) }
    });

    app.on_mark_move_f9({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[8].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 8, &app) }
    });

    app.on_mark_move_f10({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[9].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 9, &app) }
    });

    app.on_mark_move_f11({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[10].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 10, &app) }
    });

    app.on_mark_move_f12({
        let app : App = weak.upgrade().unwrap();
        let fi_copy = file_index.clone();
        let pv_copy = paths_vec.clone();
        let mut mark_move_copy = marked_move_arr[11].clone();

        move || { mark_move(pv_copy.borrow(), mark_move_copy.borrow_mut(), fi_copy.borrow(), 11, &app) }
    });

    app.run()
}