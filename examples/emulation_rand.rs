use wreq::Client;
use wreq_util::Emulation;
use strum::IntoEnumIterator;
#[tokio::main]
async fn main() -> Result<(), wreq::Error> {
    for x in Emulation::iter() {
        println!("{:?}", x);
    }
    // Build a client to emulation random devices
    let client = Client::builder()
        .emulation(Emulation::random())
        .cert_verification(false)
        .build()?;

    // Use the API you're already familiar with
    let text = client
        .get("https://tls.peet.ws/api/all")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text);

    Ok(())
}
