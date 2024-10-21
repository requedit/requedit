// Command to open a new window
// #[tauri::command]
// fn open(app_handle: tauri::AppHandle, url: String) {
//     let new_window = tauri::WindowBuilder::new(
//         &app_handle,
//         "Login",                           // the window label
//         tauri::WindowUrl::App(url.into()), // the URL to load
//     )
//     .title("Login")
//     .inner_size(800.0, 600.0) // Set the size of the new window
//     .resizable(true) // Set whether the window is resizable
//     .build()
//     .expect("failed to create new window");

//     // Optionally do something with the window, like showing it
//     new_window.show().expect("failed to show window");
// }
