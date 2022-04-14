use dogstatsd::{Options, Client};
use rand_distr::{Normal, Distribution};
use std::time::{Instant, Duration};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = std::env::var("DD_ADDR")?;
    let delay_ns: u64 = std::env::var("DELAY_NS")?.parse()?;
    let delay = Duration::from_nanos(delay_ns);
    
    let suffix = std::env::var("SUFFIX").ok().unwrap_or("".to_string());
    let name = format!("gaussian{suffix}");
    
    let options = Options::new("127.0.0.1:9000", &addr, "sujay-test", vec![]);
    let mut client = Client::new(options).await?;

    let mut rng = rand::thread_rng();
    let normal = Normal::<f64>::new(100., 10.)?;
    let mut i = 0;
    let mut start = Instant::now();

    loop {
        if i % 20 == 0 && start.elapsed() > Duration::from_secs(1) {
            let now = Instant::now();
            println!("{:02}/s", (i as f64) / (now - start).as_secs_f64());
            i = 0;
            start = now;
        }        
        let sample = format!("{}", normal.sample(&mut rng));
        client.distribution(&name, &sample, &["tag:sup"])?;
        i += 1;

        if delay_ns > 0 {
            std::thread::sleep(delay);
        }
    }

    Ok(())
}
