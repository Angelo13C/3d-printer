# Electronics
This is the electrical control board on which the firmware runs.

The board receives stable `24V` from an external power supply unit and requires around `16A` of current at peak, but during almost all of the time it requires around `10A`
of current (because the heaters are not fully on after having reached the target temperature).

On the board there's a switching regulator that converts the `24V` to `5V`, which is used by the `endstops`, the `BLTouch` and by the linear regulator
that instead converts the `5V` to `3.3V` which is used by the [ESP32-S3-WROOM-1](https://www.espressif.com/sites/default/files/documentation/esp32-s3-wroom-1_wroom-1u_datasheet_en.pdf)
microcontroller, the [MT29F2G01ABAGDWB-IT](https://datasheet.lcsc.com/lcsc/1912111437_Micron-Tech-MT29F2G01ABAGDWB-IT-G_C410863.pdf) NAND flash memory.

## Schematic
This is the electrical schematic of the board:

![Schematic](https://github.com/Angelo13C/3d-printer/assets/55251189/a8f42ddd-891c-41f4-b030-fd985ccf64f9)

## PCB
_Still under development..._

## Possible improvements
Some ideas for further improvement are:
- Add some little LEDs on the board to show that the power rails are good.
- Use an external Sigma-Delta ADC to improve the accuracy of the temperatures' readings (or calibrate the microcontroller's internal ADC).
