# miner-key: 波卡系矿工批量账号工具

[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)


-------



## Usage

### 安装

下载 [miner-key-1.0.0.zip](https://github.com/big-miner/miner-key/releases/tag/release-1.0.0)

```html

unzip miner-key-1.0.0.zip
cd  miner-key-1.0.0
./install.sh
```


### 生成一个substrate账号

```html
mkey generate --words 24 --network substrate
```

### 生成一个phala账号

```html
mkey generate --words 24 --network phala
```

### 生产一批账号到一个文件中


```html
mkey generate --words 24 --network phala -f csv -a 100 > pha.miner.csv  # 批量生成100个账号到 pha.miner.csv
```



```html
# generate substrate account
mkey generate --words 24 --network substrate

# generate phala account
mkey generate --words 24 --network phala
{
    "account_id": "0x7cd0436280575715045df7a41a0026e4129a767c0174307cf5df072c40145a38",
    "public_key": "0x7cd0436280575715045df7a41a0026e4129a767c0174307cf5df072c40145a38",
    "secret_phrase": "limb need settle stock squeeze mushroom rookie basic always boost tree stand miracle also close lend glad food bridge empty outdoor help duty must",
    "secret_seed": "0xbdf6e41161eb81d76a562bb4d68a033bfc09a367ed372a4572ffbfa2ed81c6ff",
    "ss58": "43SSZSboBVysBLaXE64SJngTzKgVkdPaRgj2MZWAuszGpEp9"
}
# batch generate phala account pha.miner.csv
mkey generate --words 24 --network phala -f csv -a 100 > pha.miner.csv  # 批量生成100个账号到 pha.miner.csv
```


## develop

```shell
make install
```