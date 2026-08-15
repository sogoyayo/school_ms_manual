use school_ms_manual::domain::ids::TeacherId;

const BANNER: &str = "\
+--------------------------------------------+
|    WEB3 SCHOOL MANAGEMENT SYSTEM           |
+--------------------------------------------+";

fn main() {
    println!("{BANNER}");
    println!("  first teacher: {}", TeacherId::from_number(1));
}