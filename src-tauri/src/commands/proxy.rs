use crate::config;
use std::process::Command;
use tauri;

#[tauri::command]
pub(crate) fn set_sys_proxy() {
    let config = config::load().unwrap();
    let address = &config.address;
    let port = &config.port.to_string();
    let http_proxy_status = Command::new("networksetup")
        .arg("-setwebproxy")
        .arg("Wi-Fi") // 网络接口，例如 "Wi-Fi" 或 "Ethernet"
        .arg(address) // 代理服务器地址
        .arg(port) // 代理服务器端口
        .status()
        .expect("failed to execute process");

    // 检查是否成功设置 HTTP 代理
    if http_proxy_status.success() {
        println!("HTTP 代理已成功设置");
    } else {
        eprintln!("设置 HTTP 代理失败");
    }

    // 设置 HTTPS 代理
    let https_proxy_status = Command::new("networksetup")
        .arg("-setsecurewebproxy")
        .arg("Wi-Fi")
        .arg(address) // 代理服务器地址
        .arg(port) // 代理服务器端口
        .status()
        .expect("failed to execute process");

    // 检查是否成功设置 HTTPS 代理
    if https_proxy_status.success() {
        println!("HTTPS 代理已成功设置");
    } else {
        eprintln!("设置 HTTPS 代理失败");
    }
}

#[tauri::command]
pub(crate) fn clean_sys_proxy() {
    // 关闭 HTTP 代理
    let http_proxy_status = Command::new("networksetup")
        .arg("-setwebproxystate")
        .arg("Wi-Fi") // 网络接口，例如 "Wi-Fi" 或 "Ethernet"
        .arg("off") // 关闭 HTTP 代理
        .status()
        .expect("failed to execute process");

    // 检查是否成功关闭 HTTP 代理
    if http_proxy_status.success() {
        println!("HTTP 代理已成功关闭");
    } else {
        eprintln!("关闭 HTTP 代理失败");
    }

    // 关闭 HTTPS 代理
    let https_proxy_status = Command::new("networksetup")
        .arg("-setsecurewebproxystate")
        .arg("Wi-Fi") // 网络接口
        .arg("off") // 关闭 HTTPS 代理
        .status()
        .expect("failed to execute process");

    // 检查是否成功关闭 HTTPS 代理
    if https_proxy_status.success() {
        println!("HTTPS 代理已成功关闭");
    } else {
        eprintln!("关闭 HTTPS 代理失败");
    }
}

#[tauri::command]
    pub(crate) fn get_proxy_status() -> (bool, bool) {
        // 检查 HTTP 代理状态
        let http_proxy_output = Command::new("networksetup")
            .arg("-getwebproxy")
            .arg("Wi-Fi") // 网络接口，例如 "Wi-Fi" 或 "Ethernet"
            .output()
            .expect("failed to execute process");

        let http_proxy_status = String::from_utf8_lossy(&http_proxy_output.stdout);
        let http_enabled = http_proxy_status.contains("Enabled: Yes");

        // 检查 HTTPS 代理状态
        let https_proxy_output = Command::new("networksetup")
            .arg("-getsecurewebproxy")
            .arg("Wi-Fi")
            .output()
            .expect("failed to execute process");

        let https_proxy_status = String::from_utf8_lossy(&https_proxy_output.stdout);
        let https_enabled = https_proxy_status.contains("Enabled: Yes");

        (http_enabled, https_enabled)
    }
