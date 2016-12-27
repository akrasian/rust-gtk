extern crate gtk;

mod example {
    use std::path::Path;
    use std::path::PathBuf;
    use gtk;
    use gtk::{WidgetExt, ButtonsType, DialogFlags, MessageType, MessageDialog, 
        Window, WindowType, Builder, Inhibit, DialogExt, Button, Toolbar, ImageMenuItem,
        ButtonExt
    };

    pub fn showmsg(){
        MessageDialog::new(None::<&Window>,
            DialogFlags::empty(),
            MessageType::Info,
            ButtonsType::Ok,
            "Hello World").run();
    }
    
    pub fn main(){
        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
        
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("ui/mainwindow.glade");
        //~ let path = Path::new("ui/mainwindow.glade");
        let builder: Builder = Builder::new_from_file(path);
        
        let window: Window = builder.get_object("window1").unwrap();
        
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
        });
        
        let btn: Toolbar  = builder.get_object("toolbar1").unwrap();
        let btn: Button  = builder.get_object("button1").unwrap();
        btn.set_label("Hello");
        btn.connect_clicked(|_| {
            println!("Clicked");
        });
        
        window.show_all();
        gtk::main();
    }
}

fn main() {
    example::main()
}
