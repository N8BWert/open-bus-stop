# Informal Bus Display

## Motivation

While traveling abroad, I noticed that a lot of cities (specifically in the Netherlands) have displays at bus stops detailing the next bus arrival.  This is relatively unheard of in the United States (except for at certain Bus Rapid Transit stations) so I figured it would be interesting to create an informal bus display.  This informal bus display is a simple LCD screen attached to a low-powered microcontroller, but we can use the RTC to try to display the next bus arrivals.

## Description

To make the bus riding experience more enjoyable and reliable, a small low-powered microcontroller display can be created to display the static arrival times of buses.  These displays can be created inexpensively and placed throughout the city informally by residents to aid bus riders in knowing when their next bus is scheduled.

To further extend this concept, our team is also looking to create a set of "towers" (raspberry pis) that can relay real-time bus arrival schedules to the low-powered displays via LoRA radios as well as creating a mapping software to display the location of the various bus-stop displays and "towers" to visualize the adoption of the informal bus stop displays.

## Technical Details

A small low-power microcontroller can be attached at each bus stop around the city.  The microcontroller will have the static table of the bus arrival times for the given bus stop.  Additionally, to improve the accuracy of the microcontroller displays a second “tower” can be created from a raspberry pi and a LoRA-capable radio.  The raspberry pi can ping the relevant transit agency’s API to determine the real-time bus arrival times for all bus stations in its local range.  Then it can relay this information via a LoRA radio to the low-power microcontroller displays to give them real-time bus arrival data.  To contribute to the formation of an informal network of the bus displays and towers a web app can be created.  The towers will relay their GPS coordinate as well as the bus stops they are serving to the web application which can create an autonomic map of the serviced area of the informal bus displays.  Additionally, the low-power microcontrollers will still be able to display static bus arrival times when not connected via LoRA, but will need to be manually registered with the central website to be displayed.
