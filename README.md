# procdump_examples

## Features

Program need to be built with debug information.
```json
"cacheVariables": {
    "CMAKE_BUILD_TYPE": "Debug"
}
```

It works with both GCC and VisualStudio! (program have to has a GUI)

LLDB can also debug GCC/VS crash dumps! (bt)

WinDbg can also show line info for Exception with GCC-built exe! (double click)

CodeLLDB can also show line info for VS crash dumps! (hit `pause`)

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Custom launch",
    "targetCreateCommands": [
        "target create ${workspaceFolder}/access_violation/build-debug/demo.exe --core C:/dumps/demo.exe(1).42140.dmp"
    ],
    "processCreateCommands": [
        "settings set target.run-args value1 value2 value3",
        // "process launch"
    ]
},
```

## Enable Windows Error Reporting

Show UI

![](./showui.png)

Check group policy

![](./gpo.png)

check service running

![](./service.png)

## Resuslt

<img src="./wer.png" width=500>
