# 迷你森林

## Intro
学了几天的rust, 写了一个🍅番茄钟APP, 除了倒计时之外, 这个番茄钟可以将你的专注时间同步到专注森林应用中.目前功能比较简陋, 后续或许会添加

## Usage
```
MiniForest 1.1
ch4xer <ch4xer@gmail.com>
A mini program which utilize Forest Api

USAGE:
    forest <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    check-coin
    check-dead-tree
    check-health-tree
    check-total-time
    help                 Print this message or the help of the given subcommand(s)
    start
    status
```

```bash
# start planting
./forest start --email <email> --password <password> --time <time(min)>
# check remained time
./forest status
# check user's coin
./forest check-coin --email <email> --password <password>
# check user's dead tree
./forest check-dead-tree --email <email> --password <password>
# check user's health tree
./forest check-health-tree --email <email> --password <password>
# check user's total focus time
./forest check-total-time --email <email> --password <password>
```

## Integration with waybar

Add these to your waybar's config

```
"custom/forest": {
    "exec":  "~/.config/waybar/bin/forest status",
    "format": " {}",
    "interval": 1,
    "return-type": "string",
    "on-click": "~/.config/waybar/bin/forest start --email 'xxxxx@xxx.com' --password <xxxxxx> --time 5",
    "tooltip": false,
}
```
