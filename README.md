# Albe's Journey (game server for Tibia 1.03)
This project (Albe's Journey) is a collection of tools to run, and play, on a Tibia 1.03 server. Here you will find the Tibia 1.03 game client, a server, sprite images, a DAT extractor, and a map converter to make it easy to edit the game world. It also includes instructions on how to get Windows 95.

#### Credits
This project was made only possible by the efforts of enthusiastic Tibia players who share a passion for the oldest versions of the game. It takes you back almost 30 years until the beginnings of 1997. It is estimated that less than 50 players in the entire world has played Tibia 1.03 - until now.

Back in 2001, a guy nicknamed Snyder created the world's first Tibia server emulator and open sourced it - his server later went on to inspire what we today call Open Tibia. About a decade after Snyder, another programmer called Jopirop created the "TOS Server" which was an updated version of Snyder's implementation of an old game server (targeting Tibia 6.4). Fast forward to 2022, another programmer nicknamed Rsribeiro released a new version for the server - targeting multiple protocols from Tibia 3.0 onwards. And today, in 2026, Albe's Journey is released. A game server for Tibia 1.03 - which is the earliest version of the game we have managed to get our hands on. Also a big thanks to jo3bingham for extracting the sprites!

Albe was the character name of the first Tibia player, who entered the game on January 10, 1997.

Tibia 1.03 is so different from what we're used to. The concept of NPCs, health/mana, skills, etc. did not exist. This is truly a barebones version of the game. If you expect to PK and kill monsters, look elsewhere!

## We need your help!
Up until a week ago, I had never written a line in Rust. Nor had I ever worked on anything low-level (packets). I'm a web developer at heart, I mostly do JavaScript and C# stuff. But that is not even my day-job anymore. But I did manage to solve a lot, and I learned a ton. But your help is neccessary!

I've managed to solve a lot of broken and missing features in the past week. But there are still some key things missing. So if you're interested and know programming - your help would be appreciated. I've tried to include as much information as possible by commenting the source code.

Things that need to be fixed/implemented:
- Moving objects
- Looking at objects/players
- Using objects (lever)
- Converting items (making bread)
- Correct placement of chat bubbles 
- Correctly opening containers (not only bags, but chests, barrels, drawerrs)
- Storing players in a database (SQLite might be the best option for easy portability?)
- A pixel-perfect "BODY.BMP" in the client (equipment background). The "ugly gingerbread man" (as I call him) was hand drawn by me as I was unable to find the original file. But the color tones might be off, and the size/placement.
___
### How to run the server
First of all, in this repository you will find both the client and the server (as well as oher files), which is neccessary to run it. If you're new to Github, simply hover over the green "Code" button and select "Download ZIP" to download the entire repository.

#### Windows 95
Then you will need to figure out a way to launch the old Tibia 1.03 client. It does not run on modern operating systems, so you will need a virtual machine or something else. I can personally recommend using Windows 95 by downloading it from Felix Rieseberg's Github repository here: https://github.com/felixrieseberg/windows95
It works on Linux. Windows and macOS and takes 2 minutes to setup.

You can also use winevdm to run Windows 95 on modern Windows installations (such as Windows 10 or Windows 11): https://github.com/otya128/winevdm

You can also use Windows 3.1 if you manage to find a virtual machine for that. Personally, I've tested this using the Windows 95 app by Felix Rieseberg, on Linux Mint. It worked flawlessly.

#### Rust
This server is written in Rust (a programming language). In order to run the server, install Rust by following the instructions on their website - here: https://forge.rust-lang.org/infra/other-installation-methods.html

#### Running the server
You can configure the server a bit inside the "server/server.toml" file - such as enabling or disabling debug messages, setting a Message of the Day, etc.

Once you have installed Rust, open up a terminal window inside the "server" directory. Then run the following:

```
cargo check
cargo run
```
The server is now running on your computer.

#### Connecting to the server
After you have setup Windows 95, you must move your Tibia 1.03 client over there (the entire "client" folder). Then launch the Tibia client and go into "File -> Preferences" and make sure the "Tibia-Server Address" is set to the local IP of your computer.

To find your local IP on Windows, open the command prompt and run `ipconfig /all` and find your IP, typically `192.168.1.xxx`. And if you're on Linux, run `ip a` in your terminal. Just make sure to get the local IP address of the machine that is running the server.

Then you can either create a new character ("New Game") or enter any name/password ("Journey Onward") and sign in. Enjoy!

#### Map editor
I built the map by using Remere's Map Editor ("RME") using Tibia 8.60 graphics. If you want to edit the map, you can either do so manually by editing the "map.json" file (inside the "server" directory) and placing Tibia 8.60 item IDs there. Link to RME: https://github.com/opentibiabr/remeres-map-editor

Link to Tibia 8.60 (required to get the Tibia.dat and Tibia.spr files for RME): https://mega.nz/#!WfA1kKwT!oH9hLUQEafAtWtzJJrd3gnn2TN383qpqQfrp7qqLbC0

If you choose to edit the map using Remere's Map Editor, you will need to convert it from OTBM (OpenTibia format) to JSON using the included "map-converter". That one is a Node.js app, so you will also need to install Node.js.
```
node convert.mjs
```
___
### Sprites
In the "sprites" folder you will find sprite images of all available items in Tibia 1.03. Feel free to check them out if you'd like. These files have nothing to do with the game server, so you can edit or delete them as you wish. It's just an included bonus.
___
### DAT files (item IDs in Tibia 1.03)
Tibia 1.03 does not use a "Tibia.DAT" file, instead it uses a "MUDOBJ.CLI" file - which includes the same stuff, but in a slightly different format. There you can find the item IDs of all items in the game. I've already extracted all the information from the "MUDOBJ.CLI" file (you will find it inside "dat-extractor/tibia103-dat-extracted-data.txt") - or you can extract it by yourself. I included a program in Rust that parses the file.

The file is quite confusing, as you must go back and forth between sprite images to map the item IDs. Luckily, I've already done that for you as well. You will find a complete list of item IDs inside the server!

Go inside the "server/src/map/mod.rs" file using a text editor of choice. At the bottom of the file you will find all item IDs, as well as IDs of all tiles.

Note that I have edited 1 item manually (trough of water). On the map it uses item ID 17751 and is converted to 0x062D.

The Tibia 1.03 game server uses hexadecimal values for item IDs. So for those of you that are used to Open Tibia item IDs (such as 3031 for gold) will be disappointed. Use the item ID list in the map file I mentioned above when you reference items.

___

*Sit tibi Tibia levis!*
