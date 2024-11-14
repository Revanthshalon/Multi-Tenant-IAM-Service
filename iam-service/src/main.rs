#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    iam_service::run_iam_service(None).await
}
