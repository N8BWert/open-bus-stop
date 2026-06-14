# Planning

For full production we will need these displays to effectively work without needing to be modified at all.  This means minimizing power consumption and cost, while also enabling the device to recover from full power loss.  With the current prototype, the rp2040's RTC will not recover from a full power loss meaning we'll need a replacement.  Additionally, the lcd1602 display does not have great visibility and the backlight is very costly.  Finally, We will need to use a solar panel to recharge the lipo battery powering the microcontrollers.

## ESP32-C3 Power Stats using Embassy

Active (CPU running without Wifi): ~20-30mA
Light Sleep (Embassy async wait) - ~130uA

Program Loop:
1. Check the RTC time
2. Update the display
3. Light Sleep for 30 seconds

Assuming steps one and two take 500 milliseconds, this gives us the following power draw:
Steps 1 and 2: 30mA * 500 milliseconds
Step 3: 130uA * 30 seconds

So one loop is 30.13mA at 30.5 seconds meaning we are consuming 30.13mA * 30.5s = 918.965mAs = 0.255mAh

Assuming the device was working 24 hours a day, this would be a total of 6.126mAh per day

## 6v 200mA Solar Panel Recharge

Expecting 75% efficiency and full sunlight for 4 hours a day we can recharge about 200mA * 0.75% * 4 hours = 600mA, which we will consider our absolute budget.

## LCD1602 Power Stats

For the prototype I used the LCD1602 to display the upcoming bus arrivals.  With the blue backlight it has a power draw of roughly 45-55mA.

This means over a day of it being on for 24 hours we would consume 1080mAh-1320mAh.

This far exceeds the 600mA budget of our system so we will not be able to use it.

## SSD1309 OLED Display

The display I used in my prototype consumes way too much power to work for the final product.  Another display I'm considering using is a 2.4" SSD1309 display.  This is relatively the same size as the LCD1602 display, but should have a significantly lower power draw.

The SSD1309 seems to draw roughly 20mA for my current application which would mean over 24 hours we would consume 480mAh.

This is solidly within our solar panel recharge rate (6.126 + 480) < 600mA so this is the way to go.

The main downside of the SSD1309 is it is a bit more expensive than the LCD1602, but I think that being able to actually make it through a day is more important than the price in this case.  Additionally, the LCD1602 is hard to see at different angles (and in the sun), whereas the SSD1309 OLED display is visible at all angles and in most levels of sunlight.

## SSD1306 OLED Display

The SSD1309 would be perfect for this project, but its relatively expensive for some reason.  I swear in the past I found them for ~$4-5 a piece, but they're like $12 on the low end now.  For that reason, I'm going to make a prototype with the SSD1306 I2C displays.  They're about 1x1" in display size, but they're also ~$3 a piece.  I'm not 100% certain that they're large enough to be read from near the bus stop though so I need to test that before I commit.

## RTC Selection

The DS3231 seems like the goto Real Time Clock Module for most applications I've seen on the internet so I'll probably choose that for perpetuity

## High-Level Circuit Diagram

```
Solar Cell ---> BQ24074 Solar Cell Lipo Charger ---> LiPo Battery ---> ESP32-C3 ---> SSD1309 Display
                                                                          |
                                                                          |
                                                                          |
                                                                          ---------> RTC (coin cell battery)
                                                                          |
                                                                          |
                                                                          |
                                                                          ---------> Power Switch (power on and off)
```

## Other Considerations

This device needs to be waterproof considering it will be sitting in the elements for years.  In Atlanta, the materials matter a lot considering it can get very hot and humid.  However, I need this to be able to be created by the general population so I think it will need to be able to be 3d printed and/or laser cuttable.  For this reason I will 3d print the majority of the body with white PETG (white to prevent overheating, plus there is UV resistant PETG) and create a front face out of laser cut acrylic.  These are easily accessible hobby manufacturing materials meaning that anyone will be able to make the displays.  To access the PCB for firmware updates and repairs we'll use an O-ring to mate two parts of the 3d printed enclosure.
