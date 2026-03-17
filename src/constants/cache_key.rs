// Định nghĩa tiền tố cho key để tránh xung đột
pub const URL_KEY_PREFIX: &str = "url:";

// Thời gian sống của cache (TTL) - đơn vị: giây
pub const URL_CACHE_TTL: u64 = 3600; // 1 giờ

/// Hàm tạo key hoàn chỉnh: url:{short_code}
pub fn format_url_key(short_code: &str) -> String {
    format!("{}{}", URL_KEY_PREFIX, short_code)
}