use crate::config;
use regex::Regex;
use std::process::{Command, Output};
use tauri;

#[tauri::command]
pub(crate) fn set_sys_proxy() {
    #[cfg(target_os = "macos")]
    {
        let config = config::load().unwrap();
        let address = &config.address;
        let port = &config.port.to_string();
        let http_proxy_status = Command::new("networksetup")
            .arg("-setwebproxy")
            .arg("Wi-Fi")
            .arg(address)
            .arg(port)
            .status()
            .expect("failed to execute process");


        if http_proxy_status.success() {
            log::info!("Macos http proxy set success");
        } else {
            log::warn!("Macos http proxy set fail");
        }


        let https_proxy_status = Command::new("networksetup")
            .arg("-setsecurewebproxy")
            .arg("Wi-Fi")
            .arg(address)
            .arg(port)
            .status()
            .expect("failed to execute process");


        if https_proxy_status.success() {
            log::info!("Macos https proxy set success");
        } else {
            log::warn!("Macos https proxy set fail");
        }
    }
    #[cfg(target_os = "windows")]
    {
        let proxy_address = format!("{}:{}", address, port);
        let http_proxy_status = Command::new("netsh")
            .args(&["winhttp", "set", "proxy", &proxy_address])
            .status()
            .expect("failed to execute process");

        if http_proxy_status.success() {
            log::info!("Windows http proxy set success");
        } else {
            log::warn!("Windows http proxy set fail");
        }
    }
}

#[tauri::command]
pub(crate) fn clean_sys_proxy() {
    #[cfg(target_os = "macos")]
    {
        let http_proxy_status = Command::new("networksetup")
            .arg("-setwebproxystate")
            .arg("Wi-Fi")
            .arg("off")
            .status()
            .expect("failed to execute process");

        if http_proxy_status.success() {
            log::info!("Macos http proxy clean success")
        } else {
            log::warn!("Macos http proxy clean fail")
        }

        let https_proxy_status = Command::new("networksetup")
            .arg("-setsecurewebproxystate")
            .arg("Wi-Fi")
            .arg("off")
            .status()
            .expect("failed to execute process");

        if https_proxy_status.success() {
            log::info!("Macos https proxy clean success")
        } else {
            log::warn!("Macos https proxy clean fail")
        }
    }
    #[cfg(target_os = "windows")]
    {
        let http_proxy_status = Command::new("netsh")
            .args(&["winhttp", "reset", "proxy"])
            .status()
            .expect("failed to execute process");

        if http_proxy_status.success() {
            log::info!("Windows http proxy clean success")
        } else {
            log::warn!("Windows http proxy clean fail")
        }
    }
}

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
            .map_or_else(|| 0, |m| m.as_str().parse::<u16>().unwrap_or(0));
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
    #[cfg(target_os = "macos")]
    {
        let http_proxy_output = Command::new("networksetup")
            .arg("-getwebproxy")
            .arg("Wi-Fi")
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

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("netsh")
            .args(&["winhttp", "show", "proxy"])
            .output()
            .expect("failed to execute process");
        let proxy_status = String::from_utf8_lossy(&output.stdout);

        proxy_status.contains("Proxy Server(s) are enabled")
    }
}
