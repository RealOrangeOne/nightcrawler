use gtk::{main_quit, ContainerExt, GtkWindowExt, Inhibit, WidgetExt, Window, WindowType};
use gdk::{EventKey, ModifierType};
use gdk::enums::key;
use webkit2gtk::{WebView, WebViewExt, WebViewExtManual};
use relm::{Relm, Update, Widget};
use config::Config;
use webview::build_webview;

#[derive(Msg, Debug)]
pub enum Actions {
    Quit,
    URIChanged,
    KeyPress(EventKey),
    LoadChanged,
}

pub struct Win {
    window: Window,
    webview: WebView,
    model: Config,
}

fn build_window_title(webview: &WebView) -> String {
    let uri = webview.get_uri().expect("Failed to get webview uri");
    let loading_msg = if webview.is_loading() {
        "(loading)"
    } else {
        ""
    };
    return format!("{} - {} {}", uri.as_str(), crate_name!(), loading_msg);
}

impl Update for Win {
    type Model = Config;
    type ModelParam = Config;
    type Msg = Actions;

    fn model(_: &Relm<Self>, config: Config) -> Self::Model {
        return config;
    }

    fn update(&mut self, event: Self::Msg) {
        let window = &self.window;
        let webview = &self.webview;

        match event {
            Actions::Quit => main_quit(),
            Actions::URIChanged | Actions::LoadChanged => {
                window.set_title(build_window_title(webview).as_str());
            }
            Actions::KeyPress(pressed_key) => {
                let has_alt = pressed_key.get_state().contains(ModifierType::MOD1_MASK);
                match pressed_key.get_keyval() {
                    k if k == key::Left && has_alt => {
                        webview.go_back();
                    }
                    k if k == key::Right && has_alt => {
                        webview.go_forward();
                    }
                    k if k == key::Home && has_alt => {
                        webview.load_uri(self.model.get_initial_url());
                    }
                    _ => (),
                }
            }
        };
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        return self.window.clone();
    }

    fn view(relm: &Relm<Self>, config: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let webview = build_webview(config.clone());

        window.set_title(build_window_title(&webview).as_str());

        window.add(&webview);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Actions::Quit), Inhibit(false))
        );

        connect!(relm, webview, connect_uri_changed(), Actions::URIChanged);
        connect!(
            relm,
            webview,
            connect_load_changed(_, _),
            Actions::LoadChanged
        );
        connect!(
            relm,
            window,
            connect_key_press_event(_, key),
            return (Actions::KeyPress(key.clone()), Inhibit(false))
        );

        return Win {
            window,
            webview,
            model: config,
        };
    }
}
