# miner key

## develop install 

```html
make install
```

## Usage

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

