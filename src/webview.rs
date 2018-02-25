use webkit2gtk::{WebContext, WebView, WebViewExt, WebViewExtManual};
use config::Config;

pub fn build_webview(config: Config) -> WebView {
    let context = WebContext::get_default().unwrap();
    let webview = WebView::new_with_context(&context);

    webview.load_uri(config.get_initial_url());

    return webview;
}
