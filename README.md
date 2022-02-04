minecraft-modpack-downloader
============================

When you want to play a modpack with any your favorite launcher.

Why made this?
--------------

Because I need it! I want to play a modpack with the vanilla launcher.

Installation
------------

```shell
$ cargo install --git https://github.com/DanSnow/minecraft-modpack-downloader
```

Usage
-----

```shell
$ minecraft-modpack-downloader path/to/manifest.json
```

Why Rust?
---------

Because I love Rust. It's cool.

How it work?
------------

It's using the same API endpoint from [mc-curseforge-api](https://github.com/Mondanzo/mc-curseforge-api) to get the file list. I have no idea where's API came from. But it works.

TODO
----

- [x] able to configure output path
- [x] copy overrides
