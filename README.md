# Rustsweep - Fast Rust-Based CLI Pingsweeping Tool

### Key Features:
- `Multithreaded` host enumeration
- Build in `TCP port knocking` 
- `ARP` based `information gathering`
- `OUI lookups` and support tools

### Requirements:
- [Rust](https://rustup.rs/)
- [net-tools](https://github.com/ecki/net-tools)
- [Crystal](https://crystal-lang.org/)

### Usage
##### Note: If OUI lookups are required, ensure `oui.txt` is available!
#####       `oui.txt` not found? Use `crystal run tools/oui_ieee_dbdl.cr` from the home directory

- Build Rustsweep and run a test session (basic):
```bash
bash build.sh
```

### Options
| Flag          | Description                                      |
|---------------|--------------------------------------------------|
| -p            | IP pattern (e.g. 192.168.178.x)                  |
| -e            | Enable TCP port scanning                         |
| -f            | Specify the port where the TCP scan starts       |
| -l            | Specify the port where the TCP scan ends         |
| -i            | Enable OUI lookup feature                        |
| -o            | Custom path of oui db file                       |
| -d            | ICMP request deadline                            |
| -t            | TCP handshake timeout                            |
| --http-test   | Web UI/Server test                               |

### Example
- Enable port scanning and OUI lookup (adjust private ip pattern for your network)
```bash
sudo ./target/release/rustsweep -p 192.168.178.x -e -f 1 -l 500 -i
```

### Output
![](https://github.com/f42h/rustsweep/blob/master/assets/test_run.gif)

### License
This project is published under the [MIT](https://github.com/f42h/rustsweep/blob/master/LICENSE) License. See the LICENSE file for more details.