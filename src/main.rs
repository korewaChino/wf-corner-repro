use gio::prelude::*;
use gtk::{gdk::Display, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::Window::builder()
        .application(application)
        .title("Hello, world!")
        .decorated(true)
        .css_classes(vec!["background", "csd"])
        .default_width(200)
        .default_height(100)
        .build();

    // set custom CSS

    let provider = gtk::CssProvider::new();

    provider.load_from_data(
        r#"
        window {
            /* Uncomment below to see working corner radius,
                Opaque windows cause the bug.
             */
            /* background-color: rgba(0.0, 0.0, 0.0, 0.75); */
            border-radius: 24px;
        }
        "#,
    );

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Top);

    // Push other windows out of the way
    // window.auto_exclusive_zone_enable();

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 40);
    window.set_margin(Edge::Right, 40);
    window.set_margin(Edge::Top, 20);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, false),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Set up a widget
    let box_ = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .width_request(200)
        .height_request(100)
        // make window rounded
        .css_classes(vec!["csd", "background"])
        .build();

    let label = gtk::Label::new(Some("Hello, world!"));

    box_.append(&label);

    window.set_child(Some(&box_));

    window.show()
}

fn main() {
    let application = gtk::Application::new(
        Some("xyz.cappuchino.wayfire-layershell-test"),
        Default::default(),
    );

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}
