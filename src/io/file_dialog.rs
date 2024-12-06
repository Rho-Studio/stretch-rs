use crate::io::OutputParams;
use tauri::{Manager, Window};
use tauri_plugin_dialog::DialogExt;

#[cfg(not(target_os = "ios"))]
#[tauri::command]
pub fn open_image_files<W>(window: Window<W>) -> Vec<String>
where
    W: tauri::Runtime,
{
    let files = window
        .app_handle()
        .dialog()
        .file()
        .add_filter("images", &["arw", "tiff", "tif", "jpg", "jpeg"])
        .blocking_pick_files()
        .unwrap_or_default();

    files.iter().map(|path| path.to_string()).collect()
}

#[cfg(not(target_os = "ios"))]
#[tauri::command]
pub fn choose_save_dir<W>(window: Window<W>, is_multi_output: bool) -> OutputParams
where
    W: tauri::Runtime,
{
    let Some(file_path) = window
        .app_handle()
        .dialog()
        .file()
        .add_filter(
            if is_multi_output {
                "Choose file format"
            } else {
                "Choose output file & format"
            },
            &["jpg", "tiff", "exr"],
        )
        .blocking_save_file()
    else {
        return OutputParams::default();
    };

    file_path.to_string().into()
}

#[cfg(target_os = "ios")]
#[tauri::command]
pub fn open_image_files<W>(window: Window<W>) -> Vec<String>
where
    W: tauri::Runtime,
{
    let files = window
        .app_handle()
        .dialog()
        .file()
        .add_filter("images", &["arw", "tiff", "tif", "jpg", "jpeg"])
        .blocking_pick_files()
        .unwrap_or_default();

    files.iter().map(|path| path.to_string()).collect()
}

#[cfg(target_os = "ios")]
#[tauri::command]
pub fn choose_save_dir<W>(window: Window<W>, is_multi_output: bool) -> OutputParams
where
    W: tauri::Runtime,
{
    let mut dialog = window.app_handle().dialog().file();

    if is_multi_output {
        dialog = dialog.set_title("Choose output location and format");
    } else {
        dialog = dialog.set_title("Choose output file");
    }

    let (sender, receiver) = channel::<OutputParams>();

    dialog
        // .add_filter("TIFF", &["tiff", "tif"])
        // .add_filter("JPEG", &["jpg", "jpeg"])
        // .add_filter("OpenEXR", &["exr"])
        .pick_file(move |file| {
            sender
                .send(match file {
                    Some(output) => output.to_string().into(),
                    None => OutputParams::default(),
                })
                .unwrap()
        });

    match receiver.recv() {
        Ok(output) => output,
        Err(_) => OutputParams::default(),
    }
}
