use file_rotate::{
    compression::Compression,
    suffix::{AppendTimestamp, DateFrom, FileLimit},
    ContentLimit, FileRotate, TimeFrequency,
};
use simplelog::*;

fn main() {
    let log_path = "logs/test.log".to_string();
    std::fs::File::create("logs/test.log.2022-05-01").unwrap();
    std::fs::File::create("logs/test.log.2022-05-02").unwrap();
    std::fs::File::create("logs/test.log.2022-05-03").unwrap();
    std::fs::File::create("logs/test.log.2022-05-04").unwrap();

    let log_config = ConfigBuilder::new().build();

    let log_file = FileRotate::new(
        &log_path,
        AppendTimestamp::with_format("%Y-%m-%d", FileLimit::MaxFiles(1), DateFrom::DateYesterday),
        ContentLimit::Time(TimeFrequency::Daily),
        Compression::None,
    );

    WriteLogger::init(LevelFilter::Debug, log_config, log_file).expect("logger init");

    debug!("some debug messages");
}
