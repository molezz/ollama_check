use std::error::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct TagsResponse {
    models: Vec<Model>,
}

#[derive(Debug, Deserialize)]
struct Model {
    name: String,
}

// 修改 check_ollama_server 函数返回值类型，错误时返回 (url, Error)
async fn check_ollama_server(client: &Client, url: String) -> Result<(String, Vec<String>), (String, Box<dyn Error + Send + Sync + 'static>)> {
    let tags_url = format!("{}/api/tags", url);
    println!("正在检测: {}", tags_url);

    let response = client.get(&tags_url).timeout(Duration::from_secs(2)).send().await; // 这里也设置超时，虽然 Client 已经设置了，但更明确
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<TagsResponse>().await {
                    Ok(tags_response) => {
                        let model_names: Vec<String> = tags_response.models.iter().map(|model| model.name.clone()).collect();
                        println!("{} 检测成功，模型: {:?}", url, model_names);
                        Ok((url, model_names))
                    }
                    Err(e) => {
                        eprintln!("解析 {} JSON 失败: {}", url, e);
                        Err((url, Box::new(e) as Box<dyn Error + Send + Sync + 'static>))
                    }
                }
            } else {
                eprintln!("请求 {} 失败，状态码: {}", tags_url, resp.status());
                Err((url, Box::<dyn Error + Send + Sync + 'static>::from(format!("HTTP 请求失败，状态码: {}", resp.status()))))
            }
        }
        Err(e) => {
            eprintln!("请求 {} 出错: {}", tags_url, e);
            Err((url, Box::new(e) as Box<dyn Error + Send + Sync + 'static>))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // 创建 ClientBuilder 并设置超时时间为 2 秒
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?;

    let urls_content = fs::read_to_string("urls.txt").await?;
    let urls: Vec<String> = urls_content.lines().map(|line| line.trim().to_string()).collect();

    let mut tasks = Vec::new();
    for url in urls {
        let client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            check_ollama_server(&client_clone, url).await
        }));
    }

    let mut results_file = fs::File::create("result.txt").await?;
    for task in tasks {
        match task.await {
            Ok(result) => {
                match result {
                    Ok((url, model_names)) => {
                        // 判断 model_names 是否为空，不为空才写入文件
                        if !model_names.is_empty() {
                            let line = format!("{},{}\n", url, model_names.join(","));
                            results_file.write_all(line.as_bytes()).await?;
                        } else {
                            println!("{} 模型列表为空，不记录到 result.txt", url);
                        }
                    }
                    Err((url, e)) => { // 解构错误元组，获取 url 和 error
                        eprintln!("检测 {} 失败: {}", url, e);
                        if let Some(reqwest_error) = e.downcast_ref::<reqwest::Error>() {
                            if reqwest_error.is_timeout() {
                                eprintln!("{} 请求超时", url);
                            }
                        }
                    }
                }
            }
            Err(join_err) => {
                eprintln!("Task join error: {}", join_err);
                return Err(Box::new(join_err) as Box<dyn Error + Send + Sync + 'static>);
            }
        }
    }

    results_file.flush().await?;
    println!("结果已保存到 result.txt");

    Ok(())
}
