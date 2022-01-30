
## External signals

uSDHC has 14 associated I/O signals.

- The CLK is an internally generated clock used to drive the MMC, SD, and SDIO cards.
- The CMD I/O is used to send commands and receive responses to and from the card. Eight data lines (DAT7~DAT0) are used to perform data transfers between the uSDHC module and the card.
- RST is an output signal used to reset the MMC card.
- VSELECT is an output signal used to change the voltage of the external power supplier. It is optional for system implementation.

If uSDHC needs to support a 4-bit data transfer, DAT7~DAT4 can also be optional and
tied to high.

The following table describes the uSDHC external signals

| Signal  | Description                                                        | Direction    |
| ------- | ------------------------------------------------------------------ | ------------ |
| CLK     | Clock for MMC/SD/SDIO card                                         | Output       |
| CMD     | CMD line connect to card                                           | Input/Output |
| DATA7   | DAT7 line in the 8-bit mode — Not used in other modes              | Input/Output |
| DATA6   | DAT6 line in the 8-bit mode — Not used in other modes              | Input/Output |
| DATA5   | DAT5 line in the 8-bit mode — Not used in other modes              | Input/Output |
| DATA4   | DAT4 line in the 8-bit mode — Not used in other modes              | Input/Output |
| DATA3   | DAT3 line in the 4/8-bit mode or configured as card detection pin. | Input/Output |
|         | The bit may be configured as card detection pin in the 1-bit mode. |              |
| DATA2   | DAT2 line or Read Wait in the 4-bit mode                           | Input/Output |
|         | Read Wait in 1-bit mode                                            |              |
| DATA1   | DAT1 line in the 4/8-bit mode                                      | Input/Output |
|         | Also, used to detect interrupt in 1/4-bit mode                     |              |
| DATA0   | DAT0 line in all the modes                                         | Input/Output |
|         | Also, used to detect busy state                                    |              |
| RESET_B | Card hardware reset signal, active low                             | Output       |
| VSELECT | IO power voltage selection signal                                  | Output       |