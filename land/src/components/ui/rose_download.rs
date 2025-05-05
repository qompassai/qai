use leptos::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Navigator, window};

#[cfg(target_arch = "wasm32")]
fn detect_os_arch() -> (String, String) {
    let navigator = window().unwrap().navigator();
    let ua = navigator.user_agent().unwrap_or_default().to_lowercase();
    let os = if ua.contains("windows") {
        "Windows"
    } else if ua.contains("mac") {
        "macOS"
    } else {
        "Linux"
    };
    let arch = if ua.contains("arm64") || ua.contains("aarch64") {
        "arm64"
    } else if ua.contains("x86_64") || ua.contains("amd64") {
        "x86_64"
    } else {
        "x86"
    };
    (os.to_string(), arch.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_os_arch() -> (String, String) {
    ("Linux".into(), "x86_64".into())
}

#[component]
pub fn RoseDownload(cx: Scope) -> impl IntoView {
    let (os, arch) = detect_os_arch();
    let filename = format!("rose-{os}-{arch}.tar.gz");
    let download_url = format!("/rose/{}", filename);
    let install_cmd = format!("curl -LO {download_url} && tar -xzf {filename}");

    let (copied, set_copied) = create_signal(cx, false);

    let on_copy = move |_| {
        let cmd = install_cmd.clone();
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                let nav = window().unwrap().navigator();
                if let Some(clip) = nav.clipboard() {
                    let _ = clip.write_text(&cmd);
                    set_copied.set(true);
                }
            }
        });
    };

    view! { cx,
        <div class="border p-4 rounded shadow">
            <p>"Detected: " <strong>{os.clone()}</strong> " / " <strong>{arch.clone()}</strong></p>
            <a href={download_url.clone()} download>
                {format!("⬇️ Download {}", filename)}
            </a>
            <pre class="bg-gray-100 p-2 rounded">{install_cmd.clone()}</pre>
            <button class="mt-2 px-4 py-1 bg-blue-600 text-white rounded" on:click=on_copy>
                "Copy Install Command"
            </button>
            {move || if copied.get() {
                view! { cx, <span class="text-green-500 ml-2">"✅ Copied!"</span> }.into_view(cx)
            } else {
                view! { cx, <></> }.into_view(cx)
            }}
        </div>
    }
}
