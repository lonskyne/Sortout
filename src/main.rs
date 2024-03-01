use rfd::FileDialog;

slint::slint! {
    import {Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window{
        in property <string> current_folder;

        callback choose_folder_clicked <=> choose_folder_btn.clicked;

        Text { text : "Current folder: " + current_folder; }

        choose_folder_btn := Button { text: "Choose folder"; }
    }
}

fn main() {
    let mut folderPath : std::path::PathBuf = std::path::PathBuf::new();

    let app : App = App::new().unwrap();
    let weak = app.as_weak();

    app.on_choose_folder_clicked( move || {
        let app : App = weak.upgrade().unwrap();
        
        let folder = FileDialog::new()
            .set_directory("/")
            .pick_folder();

        match folder { 
            Some(f) => folderPath = f,
            None => (),
        }
    }
    );

    app.run().unwrap();
}