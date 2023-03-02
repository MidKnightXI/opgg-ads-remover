# OPGG Patcher

Patcher removing ads from the desktop application [OP.GG](https://op.gg/desktop/?utm_source=opgg&utm_medium=button&utm_campaign=global).

## Installation

### Easy way

Click on your platform:

* [Windows](https://github.com/MidKnightXI/opgg-ads-remover/releases/download/master/OPGG.Patcher_2.0.6.msi)
* [macOS](https://github.com/MidKnightXI/opgg-ads-remover/releases/download/master/OPGG.Patcher_2.0.6.dmg)

### Build it yourself

* Clone the repository: `git clone https://github.com/MidKnightXI/opgg-ads-remover.git`
* Open the directory in a shell terminal and run: `yarn && yarn tauri build`
* Double click on the executable

### Contribute

If you want to contribute to the project just fork the project and open a pull request describing the problem that you're solving.

Here are some basic steps you need to follow to contribute to the project:
- Download [OP.GG](https://op.gg/desktop/?utm_source=opgg&utm_medium=button&utm_campaign=global)
- Open where it's located `C:\Users\$USER\AppData\Local\Programs\OP.GG\ressources`
- Copy `app.asar` and `app.asar.unpacked` to the location you want
- Install the [asar tool](https://github.com/electron/asar) from electron team with npm in the folder you put the asar archive
- Open `package.json` and add
```json
"script": {
    "asar": "asar e app.asar archive"
}
```
- Run `yarn asar` and go to `archive\assets\main\main.js`
- Work on it ✨

# Contributing

<!-- Do not remove or modify this section -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/MidKnightXI"><img src="https://avatars.githubusercontent.com/u/35759490?v=4" width="100px;" alt=""/><br /><sub><b> MidKnightXI </b></sub></a><br /><a href="https://github.com/MidKnightXI/opgg-ads-remover/commits?author=MidKnightXI" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/BlossomiShymae"><img src="https://avatars.githubusercontent.com/u/87099578?v=4" width="100px;" alt=""/><br /><sub><b> Blossomi Shymae </b></sub></a><br /><a href="https://github.com/MidKnightXI/opgg-ads-remover/commits?author=MissUwuieTime" title="Code">💻</a></td>
  </tr>
</table>
