## Usb Protocol 
### Frame 
| SOF | TIME STAMP | UID | ACK | SIZE | PAYLOAD | CHECKSUM | 
|-----|------------|-----|-----|------|---------|----------| 
|0x42 | 4          | 2   | 1   |  2    | SIZE    | 1        |

### Nomeclature of UID
| Name                                    | Code   |
|-----------------------------------------|--------| 
| Error Message                           | 0x0... |
| High Level -> Low Level                 | 0x1... |
| Low Level -> High Level                 | 0x2... | 
| Bi-Directional                          | 0x3... |


### List of Message

#### Error Message
| Name | UID | Description | SIZE | ACK | Format | 
|------|-----|-------------|------|-----|--------| 
| Timeout | 0x0808 | Timeout | 0 | ✕ | | 
| Not Found | 0x0404 | Not found | 0 | ✕ | |
| I'm a teapot | 0x0418 | Around the world | 0 | ✕ | |

#### High Level -> Low Level
| Name | UID | Description | SIZE | ACK | Format | 
|------|-----|-------------|------|-----|--------| 
| Bye (Disconnect) | 0x1099 | Graceful Shutdown | 0 | ✓ |  | 

#### Low Level -> High Level
| Name | UID | Description | SIZE | ACK | Format | 
|------|-----|-------------|------|-----|--------| 
| Odometer Raw Data | 0x2000 | Raw Data | 16 | ✕ | Enc Motor 1 (4L) - ... - Enc Motor 4 (4L))

#### Bi-Directional
| Name | UID | Description | SIZE | ACK | Format | 
|------|-----|-------------|------|-----|--------| 
| Hello| 0x3000| Simple Hello | 0 |  ✓ |  | 
| Heartbeat | 0x3042 | Ping | 0 | ✓ |  | 
| ACK | 0x3678 | Acknowledgement | 6 | ✕ | UID (2X) - Timestamp (4L) | 
