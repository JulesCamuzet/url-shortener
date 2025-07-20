use tracing_appender::rolling;

pub fn setup_http_tracing() {
    let log_file = rolling::daily("./logs/http", "debug");
    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
