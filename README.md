# Ollama 服务器批量检测工具

**项目简介**

本工具是一个使用 Rust 编写的命令行程序，用于批量检测 Ollama 服务器的可用性，并获取每个服务器上可用的模型列表。  它能够从 `urls.txt` 文件中读取 Ollama 服务器地址列表，逐个检测这些服务器是否可以正常访问，并尝试获取 `/api/tags` 接口返回的模型信息。 检测结果（包括服务器地址和模型列表）会被保存到 `result.txt` 文件中。

**主要功能**

*   **批量检测:**  可以一次性检测 `urls.txt` 文件中列出的多个 Ollama 服务器。
*   **服务器可用性检测:**  通过访问 `/api/tags` 接口判断服务器是否在线并响应请求。
*   **模型列表获取:**  成功连接的服务器，程序会解析 `/api/tags` 接口返回的 JSON 数据，提取模型名称列表。
*   **超时处理:**  为请求设置了超时时间，避免程序因个别服务器无响应而长时间卡顿。
*   **错误处理:**  详细记录检测过程中遇到的错误，例如连接失败、JSON 解析错误、超时等，方便用户排查问题。
*   **结果输出:**  检测结果以 CSV 格式（逗号分隔）保存到 `result.txt` 文件，方便后续处理和分析。
*   **空模型列表过滤:**  对于模型列表为空的 Ollama 服务器，不会将其记录到 `result.txt` 文件中，仅记录包含模型的服务器。

**使用方法**

1.  **准备 `urls.txt` 文件:**
    在程序所在的目录下，创建一个名为 `urls.txt` 的文本文件。  每行写入一个 Ollama 服务器的地址，例如：

    ```text
    http://localhost:11434
    http://your_server_ip:11434
    http://another_server:11434
    ```


2.  **二进制运行:**
    release下载对应平台文件， 并在目录下准备 `urls.txt` 的文本文件。
    直接运行 `ollama_check`


3.  **或构建运行:**
    确保你已经安装了 Rust 编程环境 (包括 Rustup 和 Cargo)， 并在目录下 `urls.txt` 的文本文件。 
   
    ```bash
    cargo run
    ```

    程序运行时，会在终端输出检测进度和错误信息。

4.  **查看结果:**
    程序运行结束后，检测结果会保存在程序所在目录下的 `result.txt` 文件中。  `result.txt` 文件为 CSV 格式，每行代表一个成功检测的 Ollama 服务器，包含两列：服务器 URL 和模型列表，模型名称之间用逗号分隔。 例如：

    ```text
    http://localhost:11434,llama2,mistral,codellama
    http://your_server_ip:11434,gemma,phi
    ```

    如果服务器检测失败或模型列表为空，则不会在 `result.txt` 中记录，但会在终端输出相应的错误或提示信息。

**依赖**

*   **Rust:**  本程序使用 Rust 编程语言开发。 你需要安装 Rust 编译环境才能构建和运行此程序。  建议使用 [Rustup](https://rustup.rs/) 安装和管理 Rust 版本。
*   **Cargo:**  Rust 的包管理器和构建工具。  Cargo 随 Rustup 一起安装。

**备注**

*   程序默认超时时间为 2 秒，如果你的网络环境较差或服务器响应较慢，可以考虑修改代码中的超时时间设置。


