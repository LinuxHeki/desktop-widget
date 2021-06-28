use gtk::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn page(header: &gtk::Widget) -> Option<gtk::Widget> {
    let gnome_shell_data_dir = gnome_shell_data_dir();
    let extensions_source = gnome_shell_data_dir.join("extensions");
    let extensions_backup = gnome_shell_data_dir.join("extensions.bak");
    let extensions_notified = extensions_backup.join("notified");

    if !extensions_backup.exists() || extensions_notified.exists() {
        return None;
    }

    let _ = File::create(extensions_notified);

    let label = fomat!(
        "Manually installed GNOME Shell extensions are disabled to ensure upgrades are reliable. "
        "The extensions are not usually tested as part of Pop!_OS and can cause issues. "
        "You can manually re-enable them one at a time to ensure compatibility of each "
        "extension. To re-enable, install them again from "
        "<a href=\"https://extensions.gnome.org/\">extensions.gnome.org</a>, or restore them from the backup "
        "directory.\n\nYour GNOME Shell extensions have been moved from:\n"
        [extensions_source]
        "\n\nTo this backup folder:\n"
        [extensions_backup]
    );

    let description = gtk::LabelBuilder::new()
        .justify(gtk::Justification::Center)
        .label(&label)
        .selectable(true)
        .use_markup(true)
        .wrap(true)
        .build();

    description.connect_activate_link(|_, uri| {
        let _ = std::process::Command::new("xdg-open").arg(uri).status();
        gtk::Inhibit(true)
    });

    let image = crate::scaled_image_from_resource("/org/pop/desktop-widget/extension.png", 192)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .margin_top(32)
        .build();

    Some(
        (cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 0);
            ..set_halign(gtk::Align::Center);
            ..add(header);
            ..add(&description);
            ..add(&image);
        })
        .upcast(),
    )
}

pub fn title() -> String {
    // TODO: Localize
    String::from("GNOME Shell Extensions Update")
}

fn gnome_shell_data_dir() -> PathBuf {
    glib::get_user_data_dir().expect("XDG_DATA_HOME path not found").join("gnome-shell")
}