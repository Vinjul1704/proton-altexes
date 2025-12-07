# proton-altexes
`proton-altexes` ("Proton Alternative EXEs") is a simple GUI helper that allows you to run games via Proton with different EXEs from the default. It can make dealing with multiple game installation or versions easier, for example for modding or speedrunning.

To use it, compile it or download the latest release, then set the launch options of the game you want to use it for to the following (add things like `mangohud` before `%command%`):

```
/path/to/proton-altexes %command%
```

After that, simply launch the game through Steam and you will see a GUI pop up. In there, you can add and remove EXEs to the list to run, as well as run the default EXE. Alternative EXEs are stored in a config file next to the default EXE.

By default, `proton-altexes` will close after you run an EXE. If you want to keep it open, you can use the `--keep-open` argument like this:

```
/path/to/proton-altexes --keep-open %command%
```