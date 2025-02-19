use clap::Parser;
use libc::{
    mmap, munmap, sysconf, MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE, _SC_PAGESIZE,
};
use std::ptr;

const GB: usize = 1024 * 1024 * 1024;

#[derive(Parser)]
#[command(
    version,
    about = "Mincore performance test.",
    long_about = "Mincore syscall performance test with varying region size and percentage of pages touched."
)]
pub struct MincorePerfArgs {
    #[arg(short, long, value_name = "REGION_SIZE", default_value = "1")]
    pub region_size: Option<i32>,
    #[arg(short, long, default_value = "25", value_name = "PERCENT_PAGES")]
    pub percentage_pages: Option<i32>,
}

fn main() {
    // Get system page size.
    let page_size = unsafe { sysconf(_SC_PAGESIZE) as usize };

    // Get region size from command line.
    let args = MincorePerfArgs::parse();
    let region_size = args.region_size.unwrap() as usize * GB;
    let percentage_pages = args.percentage_pages.unwrap();

    let num_pages = region_size / page_size;

    // Allocate `region_size` anonymous memory.
    let addr = unsafe {
        mmap(
            ptr::null_mut(),
            region_size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        eprintln!("Failed to mmap memory.");
        return;
    }

    // Based on the percentage of pages to touch,
    // calculate how many pages to skip when touching from the region size.
    let skip_pages = num_pages / (num_pages / (100 / (percentage_pages)) as usize) as usize;

    // Touch pages.
    for i in (0..num_pages).step_by(skip_pages) {
        unsafe {
            // Write to the page to make it dirty.
            *(addr as *mut u8).add(i * page_size) = 131;
        }
    }

    // Use mincore to check if pages are present in memory.
    // This will count accessed and dirty pages.
    let mut present_count = 0;

    // Result vector to store the result of mincore.
    let mut vec = vec![0u8; num_pages];

    // Get start time.
    let start = std::time::Instant::now();
    // Run mincore 100 times.
    for _ in 0..100 {
        unsafe {
            // Call mincore for the whole region and gather results.
            let res = libc::mincore(addr, region_size, vec.as_mut_ptr());
            // Count the number of pages that are present in memory.
            if res == 0 {
                present_count = vec.iter().filter(|&&x| x > 0).count();
            }
        }
    }

    // Print time taken in nanoseconds.
    println!("Time taken: {:?}", start.elapsed().as_nanos() / 100);
    //println!("Present pages: {}", present_count);

    // Due to rounding errors, the present count may be off by 1.
    assert!(present_count.abs_diff(num_pages / skip_pages) < 2);

    unsafe {
        munmap(addr, region_size);
    }
}
