# blue-pill-rtic-quickstart

**A quickstart for RTIC projects**

Quickstart a Rust project for the [STM32F103 (blue pill)](https://wiki.stm32duino.com/index.php?title=Blue_Pill).

## Features
- Ready to use RTIC (formerly known as RTFM)
- Uses probe.rs toolchain with cargo flash and embed
- Uses RTT for passing messages to the host
- Works on both STM32F103 and CS32F103 boards

## Setting up your machine

### Hardware
- [STM32F103C8 (Blue pill)](https://www.aliexpress.com/w/wholesale-stm32f103c8t6.html?&SortType=total_tranpro_desc) 
- [ST-Link V2](https://www.aliexpress.com/w/wholesale-st-link-v2.html?SortType=total_tranpro_desc) 
- A Windows or *nix system

### Software
- [cargo embed](https://probe.rs/guide/1_tools/cargo-embed/) (and/Or [cargo flash](https://probe.rs/guide/1_tools/cargo-flash/))

## Setting up ST-Link

First connect your ST-Link to your microcontroller, then connect the ST-Link to your computer. See the printed header pins and match them with your ST-Link pins.

When you have connected, open a terminal at your project root. To start the software and begin showing output, type

```
cargo embed --release
```

This will compile and flash your microcontroller, and show output from `rprintln!` macros.


## Trouble Shooting

### Wrong connection of the ST-Link

The pin mapping which is shown on the outer shell of your ST-Link might not be correct. Check the pin mapping by removing the ST-Link's shell, and check if the pin mapping printed on its PCB matches the mapping printed on the outer case.  If they differ, then use the mapping printed on the PCB.

If you're unable to remove the shell or the PCB is not readable, you can try one of these pin mappings which are known to exist:

|pin|      |pin|       | 
|---|------|---|-------|
| 1 | RST  | 2 | SWDIO |
| 3 | GND  | 4 | GND   |
| 5 | SWIM | 6 | SWCLK |
| 7 | 3.3V | 8 | 3.3V  |
| 9 | 5.0V |10 | 5.0V  |

|pin|      |pin|       | 
|---|------|---|-------|
| 1 | RST  | 2 | SWCLK |
| 3 | SWIM | 4 | SWDIO |
| 5 | GND  | 6 | GND   |
| 7 | 3.3V | 8 | 3.3V  |
| 9 | 5.0V |10 | 5.0V  |

### JtagNoDeviceConnected

If you receive an error when flashing that contains a message like `JtagNoDeviceConnected`, I have found the solution is the following. This assumes your microcontroller is properly hooked up to the ST-Link.

1. Unplug the ST-Link from your computer
2. Hold down the "Reset" button on your microcontroller
3. Plug the ST-Link back into your computer while holding the "Reset" button
4. While continuing to hold the "Reset" button, try `openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "init; reset halt; stm32f1x mass_erase 0; exit"` for real STM chips, or `openocd -f interface/stlink-v2.cfg -f ./cs32f1x.cfg -c "init; reset halt; stm32f1x mass_erase 0; exit"` for clone CS32 boards.)
5. When openocd is looking for the device, release the "Reset" button
6. Erasing the device should have succeeded. Try using cargo embed or flash again.

### MCU in low power state

If the software which was already flashed to the Blue pill has put the processor core into a low power state, then this prevents RTT from working. You might see an error like `jtag status contains invalid mode value - communication failure`. You might need to follow the same instructions shown [here](#jtagnodeviceconnected) to erase your device. To work around this, an idle loop is present that prevents the chip from going into a low power state.

## Sources

This quickstart is inspired by the [cortex-m-quickstart](https://github.com/japaric/cortex-m-quickstart) and [Discovery](https://rust-embedded.github.io/discovery/). I recommend reading them.

Much of this repository has been informed and modified from the original repository by TeXitoi, [Blue Pill Quickstart](https://github.com/TeXitoi/blue-pill-quickstart).