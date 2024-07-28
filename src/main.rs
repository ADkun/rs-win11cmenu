use std::io::{self, Write};
use std::process::Command;

fn main() {
    // 检查 Windows 版本
    if !is_windows_11() {
        println!("该程序仅适用于 Windows 11。");
        println!("按任意键继续...");
        let _ = io::stdin().read_line(&mut String::new()); // 等待用户输入
        return;
    }

    // 显示菜单
    loop {
        println!("Windows 11 右键菜单样式更改工具 V1.0");
        println!();
        print!("当前的状态：");
        match is_win10_style() {
            true => println!("【Win10样式】"),
            false => println!("【Win11样式】"),
        }
        println!();
        println!("使用说明：");
        println!("- 请输入【1】设置为 Windows 10 样式");
        println!("- 输入【2】还原为 Windows 11 样式");
        println!("- 输入【h】查看帮助");
        println!("- 输入【0】退出");
        println!();

        let mut input = String::new();
        io::stdout().flush().unwrap(); // 确保输出缓冲区被刷新
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                if is_win10_style() {
                    println!("当前已经为 Windows 10 样式。");
                } else {
                    set_to_win10_style();
                }
            }
            "2" => {
                if !is_win10_style() {
                    println!("当前已经为 Windows 11 样式。");
                } else {
                    restore_to_win11_style();
                }
            }
            "h" => {
                println!("更改右键菜单样式后，您可以做以下操作使更改生效：");
                println!("1. 右键点击任务栏。");
                println!("2. 选择“任务管理器”。");
                println!("3. 在“进程”中找到“Windows 资源管理器”。");
                println!("4. 右键点击“Windows 资源管理器”。");
                println!("5. 点击“重新启动”。");
            }
            "0" => {
                println!("程序退出。");
                break;
            }
            _ => println!("无效输入，请输入 1、2 或 0。"),
        }
        // 按任意键继续
        println!("按任意键继续...");
        let _ = io::stdin().read_line(&mut String::new()); // 等待用户输入
    }
}

fn is_windows_11() -> bool {
    let output = Command::new("cmd")
        .args(&["/C", "ver"])
        .output()
        .expect("无法获取 Windows 版本信息");

    let version_info = String::from_utf8_lossy(&output.stdout);
    version_info.contains("10.0.22000") || version_info.contains("10.0.22631") // Windows 11 的版本号
}

fn is_win10_style() -> bool {
    // 检查注册表项以判断是否为 Windows 10 样式
    let output = Command::new("reg")
        .args(&[
            "query",
            r"HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
        ])
        .output()
        .expect("无法查询注册表");

    let result = String::from_utf8_lossy(&output.stdout);
    result.contains("InprocServer32") // 如果存在这个项，说明是 Windows 10 样式
}

fn set_to_win10_style() {
    let _output = Command::new("reg")
        .args(&["add", r"HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32", "/f", "/ve"])
        .output()
        .expect("无法设置为 Windows 10 样式");

    println!("已成功设置为 Windows 10 样式。");
}

fn restore_to_win11_style() {
    let _output = Command::new("reg")
        .args(&[
            "delete",
            r"HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            "/f",
        ])
        .output()
        .expect("无法还原为 Windows 11 样式");

    println!("已成功还原为 Windows 11 样式。");
}
