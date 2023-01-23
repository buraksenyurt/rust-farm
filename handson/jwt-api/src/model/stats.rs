use crate::error::handler::Result;
use log::info;
use warp::reply::html;
use warp::Reply;

// Sadece admin yetkisinde olanların görebileceği demo HTML sayfasını üreten fonksiyon
pub async fn get_salary_stats(username: String) -> Result<impl Reply> {
    info!("This is a private zone. Only admins.");

    Ok(html(format!(
        r#"
            <html>
                <head>
                    <title>Salary Statistics</title>
                </head>
                <body>
                    <h1>Salary Statistics</h1>
                    <div>Wellcom {}</div>
                </body>
            </html>
        "#,
        &username
    )))
}
