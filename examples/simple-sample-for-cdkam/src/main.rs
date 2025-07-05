use simple_sample_for_cdkam::run;

#[inline]
pub fn main() -> std::result::Result<(), i32> {
    #[expect(clippy::print_stderr, reason = "use anyhow::Result interface")]
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        Err(1)
    } else {
        Ok(())
    }
}
