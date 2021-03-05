  
# QRUST: An app to make qr codes and save em as .png files.
```

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
```
## About:
Pronounced CR-UST (like the tasy part of bread)
It's pretty straightforward to use, assuming you have <a href="https://www.rust-lang.org/tools/install">rust installed.</a>

    cargo build --release  then,
    ./target/release/qrust <ARG>


You can pass it the following ARGS:
* `wifi` > It'll make a QR code to join wifi-networks
* `regular` >It'll make whatever the heck you type in there into a regular QR code..

<br>

## ToDos:
* Error handling
* Tests

<br>

### Give thanks:
Creators of the QR code library and the Rpassword library which lets you hide your wifipassword when typing it in :)


©jer <alphastrata@gmail.com>
 

