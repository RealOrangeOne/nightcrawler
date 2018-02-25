use gtk::{main_quit, ContainerExt, GtkWindowExt, Inhibit, WidgetExt, Window, WindowType};
use webkit2gtk::WebView;
use relm::{Relm, Update, Widget};
use config::Config;
use webview::build_webview;

#[derive(Msg, Debug)]
pub enum Msg {
    Quit,
}

pub struct Win {
    window: Window,
    webview: WebView
}

impl Update for Win {
    type Model = Config;
    type ModelParam = Config;
    type Msg = Msg;

    fn model(_: &Relm<Self>, config: Config) -> Self::Model {
        return config;
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => main_quit(),
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
        let webview = build_webview(config);

        window.set_title("Window Title");

        window.add(&webview);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        return Win { window, webview };
    }
}
