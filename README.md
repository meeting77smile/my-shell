# my-shell —使用Rust语言实现的命令行解释器

## 一.功能描述

my-shell 是一个用 Rust 编写的简易命令行 解释器，主要具有以下功能：

- 命令解析
  
- 管道操作
  
- 内建命令。
  

---

## 二.WSL 环境配置步骤

1. ​**​启用WSL功能​**​：

```powershell
`wsl --install
```

这会自动安装最新版WSL和Ubuntu发行版

2. ​**​初始化Ubuntu环境​**​：

```bash
sudo apt update && sudo apt upgrade -y sudo apt install build-essential git
```

3. ​**​配置Git​**​（简化版）：

```bash
git config --global user.name "meetin77smile" 
git config --global user.email "2054865827@qq.com"
```

---

## 三.代码实现

### 主循环

- 作用：不断读取用户输入，处理命令，直到用户输入 `exit` 或 EOF（Ctrl+D）。
  
- 主要流程：
  

  1. 打印提示符 `my_shell>` 。

  2. 读取一行用户输入。

  3. 判断输入内容：

- 为空则继续下一轮循环。
  
- 为 `exit` 则退出 Shell。
  
- 其他情况调用 `execute_pipeline` 处理命令（支持管道）。
  

```rust
loop {

    print!("my_shell> ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {

        Ok(0) => { println!("\nExiting my_shell."); break; }

        Ok(_) => {

            let trimmed_input = input.trim();

            if trimmed_input.is_empty() { continue; }

            if trimmed_input == "exit" { println!("Exiting my_shell."); break; }

            execute_pipeline(trimmed_input);

        }

        Err(error) => { eprintln!("Error reading input: {}", error); }

    }

}
```

### 管道与命令执行

- 作用：解析并执行用户输入的命令，支持管道。
  
- 主要流程：
  

  1. 按 `|` 分割命令，得到每个子命令。

  2. 遍历每个子命令，依次处理：

     - 检查命令是否为空，若为空则报错并清理已启动的子进程。

     - 解析命令名和参数。

     - 特殊处理 `cd` 命令（只能单独使用，不能在管道中）。

     - 其他命令则通过 `Command` 启动子进程。

     - 管道实现：前一个命令的 `stdout` 作为下一个命令的 `stdin`。

     - 最后一个命令的输出直接继承终端。

  3. 所有子进程启动后，等待它们执行完毕，并检查返回状态。

```rust
fn execute_pipeline(line: &str) {

    let commands_str: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

    let num_commands = commands_str.len();

    let mut children: Vec<Child> = Vec::new();

    let mut previous_stdout: Option<Stdio> = None;



    for (i, command_segment) in commands_str.iter().enumerate() {

        // ... 解析命令名和参数 ...

        // ... 处理 cd 命令 ...

        let mut current_command = Command::new(command_name);

        current_command.args(&args);



        if let Some(prev_stdout_handle) = previous_stdout.take() {

            current_command.stdin(prev_stdout_handle);

        }

        if i < num_commands - 1 {

            current_command.stdout(Stdio::piped());

        } else {

            current_command.stdout(Stdio::inherit());

        }

        // ... 启动子进程，错误处理 ...

    }

    // ... 等待所有子进程结束 ...

}
```

### `cd` 命令特殊处理

- cd`：切换当前工作目录，直接影响父进程（Shell 本身），不能在管道中使用。
  
- `exit`：退出 Shell，结束主循环。
  

```rust
if command_name == "cd" {

    if num_commands > 1 {

        eprintln!("'cd' cannot be part of a pipeline.");

        // ... 清理子进程 ...

        return;

    }

    let new_dir = args.get(0).map_or_else(

        || env::var("HOME").unwrap_or_else(|_| "/".to_string()),

        |x| x.to_string()

    );

    let root = Path::new(&new_dir);

    if let Err(e) = env::set_current_dir(&root) {

        eprintln!("Error changing directory to {}: {}", new_dir, e);

    }

    return;

}
```

---

## 四.Git 提交记录

1. ​**​初始化项目​**​：

```bash
git init git commit -m "feat: 初始化Rust项目结构"
```

2. ​**​添加主循环功能​**​：

```bash
`git add src/main.rs git commit -m "第二次提交：实现主循环" 
```

3. ​**​管道与命令执行实现​**​：

```bash
git add src/main.rs git commit -m "第三次提交：实现管道与命令执行" 
```

4. ​**​特殊命令处理实现​**​：

```bash
`git add src/main.rs git commit -m "第四次提交：实现特殊命令处理"
```

---

## 五.WSL中运行步骤

1. ​**​编译项目​**​：

```bash
cargo build
```

2. ​**​运行Shell​**​：

```bash
cargo run
```

3. ​**​测试管道功能​**​：

```bash
echo "测试管道" | cargo run
```
