use std::time::{Duration, SystemTime};

use file_rotate::{
    compression::Compression,
    suffix::{AppendTimestamp, DateFrom, FileLimit},
    ContentLimit, FileRotate, TimeFrequency,
};
use simplelog::*;

fn main() {
    let log_path = "logs/test.log".to_string();
    std::fs::File::create(&log_path).unwrap();
    std::fs::File::create("logs/test.log.2022-05-01_09-45").unwrap();
    std::fs::File::create("logs/test.log.2022-05-02_09-45").unwrap();
    std::fs::File::create("logs/test.log.2022-05-03_09-45").unwrap();
    std::fs::File::create("logs/test.log.2022-05-04_09-45").unwrap();

    let houre_before = SystemTime::now() - Duration::from_secs(3600);

    filetime::set_file_mtime(
        &log_path,
        filetime::FileTime::from_system_time(houre_before),
    )
    .unwrap();

    let log_config = ConfigBuilder::new().build();

    let log_file = FileRotate::new(
        &log_path,
        AppendTimestamp::with_format(
            "%Y-%m-%d_%H-%M",
            FileLimit::MaxFiles(1),
            DateFrom::DateHourAgo,
        ),
        ContentLimit::Time(TimeFrequency::Hourly),
        Compression::None,
    );

    WriteLogger::init(LevelFilter::Debug, log_config, log_file).expect("logger init");

    debug!("some debug messages");
}
