# Control interface
This app is used to communicate with the firmware of the 3D printer from a computer, similarly to [OctoPrint](https://octoprint.org/).

I've designed the app's UI with [Figma](https://www.figma.com/) and then I made a quick prototype of the app with [Unity](https://unity.com/) (the Figma files are currently not open source).

## Menus
The app is subdivided in 3 menus:
- Files
- State
- Settings

### Files
In this menu you can upload files to the printer, see the list of files currently stored in its flash memory, delete a stored file, and start printing a file that you previously uploaded.

A planned feature that is still missing is the possibilty to preview a 3D model of each file in the list (but it requires to convert a `G-code` file to a `STL` model).

### State
In this menu you can check the current state of the printer, like the current temperature of the heated bed or the G-code commands that are currently being processed by the machine.

You can also send commands to the printer like setting a new target temperature, move the tool with the buttons in the middle or make it execute some custom G-code commands that you can write in the terminal.

### Settings
In this menu you can change the settings of some components of the machine, like the `Run current` or the `Hold current` of the `TMC2209` drivers (not implemented yet) or
the `PID gains` of the [PID controllers](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller) of the two heaters.

# Why Unity?
I've temporarily decided to develop the app with Unity because it was faster for me to make a prototype with it, but I plan in the future to remake it in a framework like Qt.

# Video
A short video of the current state of the app:

https://github.com/Angelo13C/3d-printer/assets/55251189/b0da2f92-9336-40ce-9d24-ba05e9c2b530
