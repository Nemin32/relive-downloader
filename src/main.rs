extern crate gtk;
extern crate glib;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button, ProgressBarBuilder};

mod version_control;

fn main() {
    use version_control::get_remote_version;
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("R.E.L.I.V.E. Downloader");
        window.set_default_size(350, 70);

        let app_grid = gtk::GridBuilder::new().hexpand(true).vexpand(true).build();
        let download_button = Button::with_label("Download newest version.");
        let download_progressbar = ProgressBarBuilder::new().hexpand(true).vexpand(true).valign(gtk::Align::Center).build();
        let unzip_button = gtk::Button::with_label("Unzip");

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        download_button.connect_clicked(move |_| {
            let tx = tx.clone();
            //std::thread::spawn(move || {version_control::get_remote_version(tx)});
            std::thread::spawn(move || {version_control::download_newest(tx)});
        });

        let cln = download_button.clone();
        let cln2 = download_progressbar.clone();
        rx.attach(None, move |(text, frac)| {
            cln.set_label(&text);

            use ProgressBarExt;
            cln2.set_fraction(frac);

            glib::Continue(true)
        });


        app_grid.add(&download_button);
        app_grid.add(&unzip_button);
        app_grid.attach(&download_progressbar, 0,1, 2,1);

        window.add(&app_grid);
        window.show_all();
    });

    application.run(&[]);

}
