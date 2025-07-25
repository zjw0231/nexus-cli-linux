use crate::config::Config;
// 删除colored依赖
use crate::orchestrator::{OrchestratorClient, Orchestrator};
use std::error::Error;
use std::path::PathBuf;

use crate::environment::Environment;
// 移除logging导入
use crate::key_manager;

/// 提示用户输入节点ID或创建新节点
#[allow(dead_code)]
pub async fn setup_node_id(
    _config_path: &PathBuf,
    config: &mut Config,
    orchestrator: &OrchestratorClient,
) -> Result<String, Box<dyn Error>> {
    println!("Would you like to:");
    println!("1. Enter an existing node ID");
    println!("2. Create a new node");
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("Please enter your node ID:");
            print!("> ");
            std::io::Write::flush(&mut std::io::stdout())?;
            
            let mut node_id = String::new();
            std::io::stdin().read_line(&mut node_id)?;
            let node_id = node_id.trim().to_string();
            
            // Verify the node ID exists
            // (In a real implementation, this should make an API call to check)
            println!("Adding your node ID to the CLI");
            
            Ok(node_id)
        }
        "2" => {
            // Register a new node with the orchestrator
            let user_id = &config.user_id;
            if user_id.is_empty() {
                return Err("No user ID found in config. Please register a user first.".into());
            }
            
            println!("Creating a new node ID...");
            let node_id = orchestrator.register_node(user_id).await?;
            println!("Successfully registered node with ID: {}", node_id);
            
            Ok(node_id)
        }
        _ => Err("Invalid choice".into()),
    }
}

/// 初始化Nexus环境，包括配置、日志和身份验证
#[allow(dead_code)]
pub async fn initialize_environment(
    config_path: &PathBuf,
    api_url: Option<String>,
    client_id: Option<String>,
    namespace: Option<String>,
) -> Result<Environment, String> {
    // 配置初始化
    let _config = Config::load_from_file(config_path)
        .map_err(|e| format!("配置加载失败: {}", e))?;
    
    // 日志初始化
    env_logger::init();
    
    // 创建和初始化环境
    let mut env = Environment::default();
    
    // 设置API URL
    if let Some(url) = api_url {
        env.api_url = url;
    }
    
    // 设置Client ID
    if let Some(id) = client_id {
        env.client_id = id;
    }
    
    // 设置命名空间
    if let Some(ns) = namespace {
        env.namespace = ns;
    }
    
    // 加载密钥信息
    let key_manager = key_manager::load_or_generate_signing_key()
        .map_err(|e| format!("密钥管理器创建失败: {}", e))?;
    env.key_manager = key_manager;
    
    Ok(env)
} 