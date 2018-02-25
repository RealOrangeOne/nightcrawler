use gtk::{main_quit, ContainerExt, GtkWindowExt, Inhibit, WidgetExt, Window, WindowType};
use webkit2gtk::{WebView, WebViewExt, WebViewExtManual};
use relm::{Relm, Update, Widget};
use config::Config;
use webview::build_webview;

#[derive(Msg, Debug)]
pub enum Actions {
    Quit,
    URIChanged,
}

pub struct Win {
    window: Window,
    webview: WebView,
}

fn build_window_title(pre: String) -> String {
    return format!("{} - {}", pre.as_str(), crate_name!());
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
            Actions::URIChanged => {
                window.set_title(
                    build_window_title(webview.get_uri().expect("Failed to get webview uri"))
                        .as_str(),
                );
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

        window.set_title(build_window_title(config.get_initial_url().into()).as_str());

        window.add(&webview);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Actions::Quit), Inhibit(false))
        );

        connect!(relm, webview, connect_uri_changed(), Actions::URIChanged);

        return Win { window, webview };
    }
}
