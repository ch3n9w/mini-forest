# 迷你森林

## Intro
学了几天的rust, 写了一个🍅番茄钟APP, 除了倒计时之外, 这个番茄钟可以将你的专注时间同步到专注森林应用中.目前功能比较简陋, 后续或许会添加

## Usage
```
MiniForest 1.0
ch4xer <ch4xer@gmail.com>
A mini program which utilize Forest Api

USAGE:
    forest <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    start     Start planting trees
    status    Read remained time
```

```bash
# start planting
./forest start --email <email> --password <password> --time <time(min)>
# check remained time
./forest status
```

## Integration with waybar

Add these to your waybar's config

```
"custom/forest": {
    "exec":  "~/.config/waybar/bin/forest status",
    "format": " {}",
    "interval": 1,
    "return-type": "string",
    "on-click": "~/.config/waybar/bin/forest start --email 'xxxxx@gmail.com' --password xxxxxxx --time 5",
    "tooltip": false,
}

```
