use miette::Result;

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<()> {
    pacquet_cli::run_cli().await
}
