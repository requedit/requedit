use crate::config;
use regex::Regex;
use std::process::{Command, Output};
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

// #[derive(Debug, Serialize, Deserialize)]
struct SysProxyInfo {
    enabled: bool,
    server: String,
    port: u16,
}

impl SysProxyInfo {
    fn from_command(output: Output) -> Self {
        let proxy_status = String::from_utf8_lossy(&output.stdout);
        let enabled_regex = Regex::new(r"Enabled: (\w+)").unwrap();
        let server_regex = Regex::new(r"Server: (\S+)").unwrap();
        let port_regex = Regex::new(r"Port: (\d+)").unwrap();
        // 从输出中提取并转换字段
        let enabled = enabled_regex
            .captures(&proxy_status)
            .and_then(|cap| cap.get(1))
            .map_or(false, |m| m.as_str() == "Yes");

        let server = server_regex
            .captures(&proxy_status)
            .and_then(|cap| cap.get(1))
            .map_or_else(|| "Unknown".to_string(), |m| m.as_str().to_string());

        let port = port_regex
            .captures(&proxy_status)
            .and_then(|cap| cap.get(1))
            .map_or_else(
                || 0,                                       // 当未找到端口或解析失败时，返回默认值 0
                |m| m.as_str().parse::<u16>().unwrap_or(0), // 将字符串解析为 u16，失败时返回 0
            );
        SysProxyInfo {
            enabled,
            server,
            port,
        }
    }

    fn enabled(self) -> bool {
        let c = config::get_global_config();
        if !self.enabled {
            return false;
        }
        if self.server != c.address {
            return false;
        }
        if self.port != c.port {
            return false;
        }
        true
    }
}

#[tauri::command]
pub(crate) fn get_proxy_status() -> bool {
    // 检查 HTTP 代理状态
    let http_proxy_output = Command::new("networksetup")
        .arg("-getwebproxy")
        .arg("Wi-Fi") // 网络接口，例如 "Wi-Fi" 或 "Ethernet"
        .output()
        .expect("failed to execute process");

    let http_proxy_info = SysProxyInfo::from_command(http_proxy_output);

    // 检查 HTTPS 代理状态
    let https_proxy_output = Command::new("networksetup")
        .arg("-getsecurewebproxy")
        .arg("Wi-Fi")
        .output()
        .expect("failed to execute process");
    let https_proxy_info = SysProxyInfo::from_command(https_proxy_output);

    http_proxy_info.enabled() && https_proxy_info.enabled()
}
