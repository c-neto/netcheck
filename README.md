# netcheck

CLI tool to check IP kind. The IP is given as an argument and it will be shown one of the following types:

- `broadcast`
- `documentation`
- `link_local`
- `loopback`
- `multicast`
- `private`
- `public`
- `unspecified`

---

## Usage Example

```bash
$ netcheck 10.5.6.8     
private
```

---

```bash
$ netcheck 12.5.68.2    
public
``` 

---

```bash
$ netcheck 255.255.255.255
broadcast
```

## How to Install

```bash
$ cargo install netcheck
```
