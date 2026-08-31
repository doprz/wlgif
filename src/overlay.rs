use crate::backend::{Backend, RecordConfig};
use crate::region::Region;
use anyhow::{Context, Result};
use gtk::prelude::*;
use gtk::{self, gdk, glib};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::path::Path;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

const BORDER_WIDTH: i32 = 4;

const CSS: &str = "
window {
    background-color: transparent;
}

.stop-button {
    background-color: #FF3333;
    background-image: none;
    color: white;
    border: none;
    border-radius: 14px;
    min-width: 28px;
    min-height: 28px;
    font-size: 12px;
    padding: 0;
}

.stop-button:hover {
    background-color: #FF6666;
}

.stop-button:active {
    background-color: #CC2222;
}
";

pub fn run_with_overlay(
    region: Region,
    backend: Box<dyn Backend>,
    video: &Path,
    config: &RecordConfig,
) -> Result<()> {
    let recording_done = Arc::new(AtomicBool::new(false));

    let video_owned = video.to_path_buf();
    let config_owned = RecordConfig {
        fps: config.fps,
        duration: config.duration,
        quiet: config.quiet,
    };

    let recording_done_clone = Arc::clone(&recording_done);
    let record_handle = thread::Builder::new()
        .name("wlgif-record".into())
        .spawn(move || -> Result<()> {
            let result = backend.record(Some(&region), &video_owned, &config_owned);
            recording_done_clone.store(true, Ordering::SeqCst);
            result
        })
        .context("failed to spawn recording thread")?;

    gtk::init().context("failed to initialize GTK")?;

    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("no display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let rw = region.width;
    let rh = region.height;
    let rx = region.x;
    let ry = region.y;
    let btn_area_h: i32 = 40;
    let total_w = rw as i32 + BORDER_WIDTH * 2;
    let total_h = rh as i32 + BORDER_WIDTH * 2 + btn_area_h;

    let window = gtk::Window::builder()
        .title("wlgif-overlay")
        .default_width(total_w)
        .default_height(total_h)
        .decorated(false)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, false),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    window.set_margin(Edge::Top, ry as i32 - BORDER_WIDTH - btn_area_h);
    window.set_margin(Edge::Left, rx as i32 - BORDER_WIDTH);
    window.set_margin(Edge::Right, 0);
    window.set_margin(Edge::Bottom, 0);

    window.set_exclusive_zone(-1);
    window.set_keyboard_mode(KeyboardMode::None);

    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_content_width(total_w);
    drawing_area.set_content_height(total_h);
    drawing_area.set_can_target(false);

    drawing_area.set_draw_func(move |_area, cr, _w, _h| {
        let _ = cr.set_operator(gtk::cairo::Operator::Clear);
        let _ = cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        let _ = cr.paint();

        let _ = cr.set_operator(gtk::cairo::Operator::Over);

        let rw_f = rw as f64;
        let rh_f = rh as f64;
        let bw = BORDER_WIDTH as f64;
        let y_off = btn_area_h as f64;

        let _ = cr.set_source_rgba(1.0, 0.2, 0.2, 0.9);

        let _ = cr.rectangle(0.0, y_off, rw_f + bw * 2.0, bw);
        let _ = cr.fill();

        let _ = cr.rectangle(0.0, y_off + rh_f + bw, rw_f + bw * 2.0, bw);
        let _ = cr.fill();

        let _ = cr.rectangle(0.0, y_off, bw, rh_f + bw * 2.0);
        let _ = cr.fill();

        let _ = cr.rectangle(rw_f + bw, y_off, bw, rh_f + bw * 2.0);
        let _ = cr.fill();
    });

    let main_loop = glib::MainLoop::new(None, false);

    let stop_button = gtk::Button::builder()
        .label("■")
        .css_classes(["stop-button"])
        .build();

    let main_loop_for_btn = main_loop.clone();
    stop_button.connect_clicked(move |_| {
        unsafe {
            libc::kill(libc::getpid(), libc::SIGINT);
        }
        let ml = main_loop_for_btn.clone();
        glib::timeout_add_local_once(
            std::time::Duration::from_millis(200),
            move || ml.quit(),
        );
    });

    let overlay = gtk::Overlay::new();
    overlay.set_child(Some(&drawing_area));
    overlay.add_overlay(&stop_button);
    stop_button.set_halign(gtk::Align::Center);
    stop_button.set_valign(gtk::Align::Start);
    stop_button.set_margin_top(6);

    window.set_child(Some(&overlay));

    let done = Arc::clone(&recording_done);
    let main_loop_quit = main_loop.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if done.load(Ordering::SeqCst) {
            main_loop_quit.quit();
            return glib::ControlFlow::Break;
        }
        glib::ControlFlow::Continue
    });

    window.present();

    main_loop.run();

    let record_result = record_handle
        .join()
        .unwrap_or_else(|_| Err(anyhow::anyhow!("recording thread panicked")));

    if let Err(e) = record_result {
        let msg = format!("{}", e);
        if !msg.contains("interrupted") && !msg.contains("signal") {
            return Err(e);
        }
    }

    Ok(())
}
