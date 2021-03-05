/*
         _______                   _____                    _____                    _____             _____
        /::\    \                 /\    \                  /\    \                  /\    \           /\    \
       /::::\    \               /::\    \                /::\____\                /::\    \         /::\    \
      /::::::\    \             /::::\    \              /:::/    /               /::::\    \        \:::\    \
     /::::::::\    \           /::::::\    \            /:::/    /               /::::::\    \        \:::\    \
    /:::/~~\:::\    \         /:::/\:::\    \          /:::/    /               /:::/\:::\    \        \:::\    \
   /:::/    \:::\    \       /:::/__\:::\    \        /:::/    /               /:::/__\:::\    \        \:::\    \
  /:::/    / \:::\    \     /::::\   \:::\    \      /:::/    /                \:::\   \:::\    \       /::::\    \
 /:::/____/   \:::\____\   /::::::\   \:::\    \    /:::/    /      _____    ___\:::\   \:::\    \     /::::::\    \
|:::|    |     |:::|    | /:::/\:::\   \:::\____\  /:::/____/      /\    \  /\   \:::\   \:::\    \   /:::/\:::\    \
|:::|____|     |:::|____|/:::/  \:::\   \:::|    ||:::|    /      /::\____\/::\   \:::\   \:::\____\ /:::/  \:::\____\
 \:::\   _\___/:::/    / \::/   |::::\  /:::|____||:::|____\     /:::/    /\:::\   \:::\   \::/    //:::/    \::/    /
  \:::\ |::| /:::/    /   \/____|:::::\/:::/    /  \:::\    \   /:::/    /  \:::\   \:::\   \/____//:::/    / \/____/
   \:::\|::|/:::/    /          |:::::::::/    /    \:::\    \ /:::/    /    \:::\   \:::\    \   /:::/    /
    \::::::::::/    /           |::|\::::/    /      \:::\    /:::/    /      \:::\   \:::\____\ /:::/    /
     \::::::::/    /            |::| \::/____/        \:::\__/:::/    /        \:::\  /:::/    / \::/    /
      \::::::/    /             |::|  ~|               \::::::::/    /          \:::\/:::/    /   \/____/
       \::::/____/              |::|   |                \::::::/    /            \::::::/    /
        |::|    |               \::|   |                 \::::/    /              \::::/    /
        |::|____|                \:|   |                  \::/____/                \::/    /
         ~~                       \|___|                   ~~                       \/____/
Pronounced CR-UST (like the tasy part of bread)
An app to make qr codes and save em as .png files.
Why? --> Coz i don't trust the online ones?
jer <alphastrata@gmail.com>
 */

use clap::{App, Arg, ErrorKind};
use qrcode_generator::{QRCodeError, QrCodeEcc};

fn main() -> Result<(), ErrorKind> {
    let matches = App::new("qruster")
        .version("0.1")
        .author("Jeremy Webb")
        .about("Makes QR codes from strings")
        .arg(
            Arg::with_name("INPUT")
                .help("Run the app with a cmd arg to make QR code representing that string")
                .short("i")
                .long("input")
                .required(false)
                .index(1),
        )
        .get_matches();

    let state = matches.value_of("INPUT");
    match state {
        None => println!("Error passing input, indicate whether you want regular or wifi mode"),
        Some(s) => match s {
            "wifi" => {
                println!("I need your wifi's SSID");
                let t = pull_usr_input().unwrap();

                println!("I'll need the type of Security it's using, for example WEP, or WPA2");
                let s = pull_usr_input().unwrap();

                println!("What's the password?");
                //let p = pull_usr_input().unwrap();
                let p = read_pass().unwrap();

                println!("Answer <true> or <false> is the network hidden?");
                let h = pull_usr_input().unwrap_or("false".to_string()); // Most ppl's networks will not be hidden?

                let _ = make_wifi_qr(&t[..], &s[..], &p[..], &h[..]);
            }
            "regular" => {
                let _ = make_qr_code(state.unwrap());
            }
            _ => println!("Error passing input"),
        },
    }
    Ok(())
}
fn pull_usr_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim_end().to_string())
}
fn make_wifi_qr(t: &str, s: &str, p: &str, h: &str) -> Result<(), QRCodeError> {
    let contents = format!("WIFI:S:{};T:{};P:{};H:{};", t, s, p, h);
    qrcode_generator::to_png_to_file(contents, QrCodeEcc::Low, 1024, "wifi_login.png")?;
    Ok(())
}

fn make_qr_code(input: &str) -> Result<(), QRCodeError> {
    println!("Generating QR code");
    let filename = format!("{}.png", &input.clone().to_string());
    qrcode_generator::to_png_to_file(input, QrCodeEcc::Low, 1024, filename)?;
    println!("DONE!");
    Ok(())
}

fn read_pass() -> Result<String, std::io::Error> {
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    println!("GOT IT!");
    Ok(pass)
}
