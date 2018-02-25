use webkit2gtk::{CookieAcceptPolicy, CookieManagerExt, SettingsExt, TLSErrorsPolicy, WebContext,
                 WebContextExt, WebView, WebViewExt};
use config::Config;

fn setup_webview(webview: &WebView) {
    let settings = WebViewExt::get_settings(webview).unwrap();
    settings.set_enable_developer_extras(false);
    settings.set_enable_java(false);
    settings.set_enable_page_cache(false);
    settings.set_enable_smooth_scrolling(true);
    settings.set_enable_xss_auditor(true);
    settings.set_javascript_can_access_clipboard(false);
    settings.set_javascript_can_open_windows_automatically(false);
    settings.set_media_playback_requires_user_gesture(true);
    settings.set_user_agent(crate_name!());
    settings.set_enable_offline_web_application_cache(false);
    settings.set_enable_private_browsing(true);
}

fn get_webview_context() -> WebContext {
    let context = WebContext::get_default().unwrap();

    match context.get_cookie_manager() {
        Some(manager) => {
            manager.delete_all_cookies();
            manager.set_accept_policy(CookieAcceptPolicy::Never)
        }
        None => (),
    }

    context.set_tls_errors_policy(TLSErrorsPolicy::Fail);
    context.set_spell_checking_enabled(false);

    context.clear_cache(); // Just in case something has slipped through!

    return context;
}

pub fn build_webview(config: Config) -> WebView {
    let context = get_webview_context();

    let webview = WebView::new_with_context(&context);

    webview.load_uri(config.get_initial_url());

    setup_webview(&webview);

    return webview;
}
