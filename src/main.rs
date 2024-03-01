slint::slint! {
    import {Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window{
        property <int> counter: 1;

        Text { text :"Hello world!"; }
        
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
    println!("Hello, world!");
}
