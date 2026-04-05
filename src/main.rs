use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct ClientInfo {
    ip: String,
    status: String,
    pc_name: String,
}

#[derive(Clone)]
struct AppState {
    clients: Arc<Mutex<Vec<ClientInfo>>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Web 🚀</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #fff;
            min-height: 100vh;
            margin: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            text-align: center;
        }
        h1 { font-size: 3rem; margin-bottom: 0.5rem; }
        p { font-size: 1.2rem; color: #a0a0a0; }
        .badge {
            background: #f74c06;
            color: white;
            padding: 0.3rem 0.8rem;
            border-radius: 20px;
            font-size: 0.9rem;
            margin-top: 1rem;
            display: inline-block;
            text-decoration: none;
        }
        .nav-link {
            color: #fff;
            text-decoration: none;
            margin: 0 10px;
            font-weight: bold;
        }
        .nav-link:hover {
            color: #f74c06;
        }
    </style>
</head>
<body>
    <nav style="position: absolute; top: 20px; width: 100%; text-align: center;">
        <a href="/" class="nav-link">Главная</a>
        <a href="/clients" class="nav-link">Клиенты</a>
    </nav>
    <h1>🦀 Hello from Rust!</h1>
    <p>Powered by Actix Web</p>
    <a href="/clients" class="badge">Смотреть клиентов</a>
</body>
</html>
        "#)
}

async fn clients_page(data: web::Data<AppState>) -> impl Responder {
    let clients = data.clients.lock().unwrap();
    let mut rows = String::new();
    for client in clients.iter() {
        let status_class = if client.status == "Online" { "status-online" } else { "status-offline" };
        rows.push_str(&format!(
            r#"<tr><td>{}</td><td><span class="{}">{}</span></td><td>{}</td></tr>"#,
            client.ip, status_class, client.status, client.pc_name
        ));
    }

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!(
        r#"
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Клиенты</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #fff;
            min-height: 100vh;
            margin: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
        }}
        h1 {{ margin-top: 2rem; }}
        table {{ border-collapse: collapse; width: 80%; margin: 2rem auto; background: rgba(255, 255, 255, 0.05); border-radius: 8px; overflow: hidden; }}
        th, td {{ border: 1px solid #333; padding: 1rem; text-align: left; }}
        th {{ background-color: #0f172a; color: #f74c06; }}
        tr:nth-child(even) {{ background-color: rgba(255, 255, 255, 0.02); }}
        .status-online {{ background-color: #28a745; color: white; padding: 0.2rem 0.6rem; border-radius: 4px; font-size: 0.9rem; }}
        .status-offline {{ background-color: #dc3545; color: white; padding: 0.2rem 0.6rem; border-radius: 4px; font-size: 0.9rem; }}
        .nav-link {{
            color: #fff;
            text-decoration: none;
            margin: 0 10px;
            font-weight: bold;
        }}
        .nav-link:hover {{ color: #f74c06; }}
    </style>
</head>
<body>
    <nav style="width: 100%; text-align: center; padding: 20px 0;">
        <a href="/" class="nav-link">Главная</a>
        <a href="/clients" class="nav-link">Клиенты</a>
    </nav>
    <h1>👥 Список клиентов</h1>
    <table>
        <thead>
            <tr>
                <th>IP Адрес</th>
                <th>Статус</th>
                <th>Имя ПК</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>
</body>
</html>
        "#,
        rows
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let app_state = AppState {
        clients: Arc::new(Mutex::new(vec![
            ClientInfo { ip: "192.168.1.10".to_string(), status: "Online".to_string(), pc_name: "DESKTOP-01".to_string() },
            ClientInfo { ip: "192.168.1.15".to_string(), status: "Offline".to_string(), pc_name: "LAPTOP-DEV".to_string() },
            ClientInfo { ip: "10.0.0.5".to_string(), status: "Online".to_string(), pc_name: "SERVER-MAIN".to_string() },
        ])),
    };

    println!("🚀 Starting server on port {}...", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(index))
            .route("/clients", web::get().to(clients_page))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
