use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog::o!());

    let _connection = nats::Options::with_user_pass("test", "123")
        .reconnect_callback({
            let logger = logger.clone();
            move || slog::info!(logger, "reconnected")
        })
        .disconnect_callback({
            let logger = logger.clone();
            move || slog::info!(logger, "disconnected")
        })
        .connect("127.0.0.1:4222")
        .unwrap();

    for _ in 0..200 {
        slog::info!(logger, "padding out the logs");
    }
}
