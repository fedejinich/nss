mod app;
mod state;
mod storage;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::bootstrap()?;
    ui::run(&mut app).await?;
    app.persist_state();
    Ok(())
}
