# ASCIPIX
<!-- Lean font from https://www.coolgenerator.com/ascii-text-generator -->
![](./ascipix.png)

small and ~~fast~~ **Blazingly fast!!!!!!** tool to convert images into ascii art
written in Rust 🦀

# Build
```
git clone https://github.com/almazgaliev99/ascipix
cd ascipix
cargo build --release
```

# dependencies

* image = "0.24.2"
* clap = "3.2.10"

# usage help
```
./ascipix -h
```
# usage examples
![](./cat.jpg)
```
ascipix -i ./cat.jpg --invert --scale 200

```````````````````````````````````   `` ````    ``````````  ````     
```````````````````````````````````````````  ```` `` ````  ````       
``````````````````````````````````        ``   ````  "yWWHz;`````     
`````````````````````````````````` ```````` ```````;yHHHHHHHv;`       
```````````````````````````````````  ``````````  "yWWWWWWWWHv"```     
`````````"yWWy"```````````````````  ``  ``   ``"yWWWWWWWHHHHWHv"`     
```````"zyyzyyyv"``"yHHHHHHy"``````````"yWHHHHWWWWWWWWWWHHHHWHz;`     
```````"yHyzzzzyWWWWWWHHHHHHHHHHHHHHHHHHHHHHHHHHHHWWWWWWWWHHWHz;`     
`````"zyyyyyzzzyWWWWHHWWWWHHHHHHHHHHHHHHHHHHHyyyyHHHHWWWWWHHWHv"`     
`````"yHzzyyHHHWWWWWHHWWWWWWWWHHHHyyyyyyHHyyyzzzyyyyyyHWWWWWWHv"`     
```"zHHyzzzzyyHHzzHWWWHyyyHWWWWWHHHHyyzzyyyyyyzzzyyyzzzyHWWWWHz;`     
```"yHyyyyHHyzv;";yWWHyvvvzyHWWWWWWHyyzzzyyyyyzzzzzzzzzzzyWWWHv"`     
```"yHyyyyyHHy"``;yWWHyzv;;vzyHWWWHyyzzzzzzzzzzzzzzzzzzzyHWWWHv;`     
```"yHyyyyHHHz"``"vyHWHyzv;;;zyHWWHyzzzzzzzzzzzzzzzzzzyHWWHHWHv;`     
```"yHyyyyHHHz;````;yWWWHyzvzyHWWWHyyzzzzzzzzzzzzzzzzyWWWHzzHHv"`     
```"yWHyyyyHHz"````"vyWWWWHHWWWHHHyyzzyyyyyzzzzzzzzzzzzyHHvvyyyv"     
```"yWHyyyHHHyv"````"vHWWWWWWHyzzzzzyHWWWWWHyzzzvvvvzzvvyyv;zyHHHyv;``
  `"yHyyyyzyHHHyv;;vHWWWHHHyyzzzzyHHHWWWWWWWWHz;```";zHWWHv;;vzyzv;"` 
```"zyyzzyHHHHHHWWWWWWHyyyyzzzzzvzyyyyzyHWWWWHz;`````"vzyzvvzHHHHyv;``
   ``"yWWHHHHHWWWHHHHHyyyzzzzzzz;;;;;;`";zHWWHz;"``````````"zz"```````
`````"yHHHHHWWHHHz;vHWHHHyyyyyzz;""```";zHHHyz"``"vzzv"````"vv"       
   ````;yWWWHHHyyyzyHWHyyzzyyHWHyzzv;"";vzzzz;"`";yWHy"``";zyz"`` ````
 ```` `"yHyyHHHyyzzzzzzzzzzz;""""``````````````` ```""""vzyz"``   ``  
 ````"yHHyyyyyyzzzzyyyyzzzzzzyyHHHHyv;` `````  ``````";zyz"`` `` `````
`````;yWHyzzzzzzzzyyHHyzzzzzyyyHHHzv"``````````";zyyyyyz"``````       
     "yWHyzzzzzzyyyyHyyyyyzzzzzzyyzzvvvvvvvvvvzzHHHyv"`````  ````     
 ````;yWHyyyzzzzzyyyyyzyyyyzyyyyzzzzzzzzzv;vzyyyyyyyyv"``````         
 ``"zHHHHHyyzzzzzyyyyzzzyyyyyyHHHHyyyyzv""";zyyyyyyyHHyv"` ``````     
```"yWHHHHHHyyzzzv;;;;;;vzzzyyyyHHyHHHzv""vzyy;";;;;;zHHyv" ``        
 ``"yHHyyyyyyzzzzv"``  `";vvzzyyyyyyHWHyyyHHHz"``   ``;zHy"``````     
```"zyzvzyzzzzzv;"``";vvvzyyzvvvvvvvzzyHHHyzzvv;````""`;yy"           
 ``"vv"";;;vvvvvvvvvyyyyyyyzvvv;"""""";vzz;";vHyv"";;;`"yy"``````     
  `"vv"``  ``;zHWWWWWy;````"zyz;"``  ``;zyyz;;zyHyyyv;;vzv"  ````     
 ``"vv"````";vyHHHyyyz"````"vzyyv""`"""";vyz"``"zyHHyyHz"``````       
```"vz;"""vzzz"``  ``````  ``"zzzvvvvvv;vvzv"````` `````  ````        
 ``";zyHHHyv"``````    ``````"vzyyyzvvvvzv"    ````````````  ````     
   ````       ``   ````````  ``"vyHHHHyv"```````````  `` ``````     
```

```
./ascipix -i ./cat.jpg --invert --scale 200 -g 1
```
```
...................................   .. ....    ..........  ....     
...........................................  .... .. ....  ....       
..................................        ..   ....  `yNNWx!.....     
.................................. ........ .......!yWWWWWWW?!.       
...................................  ..........  `yNNNNNNNNW?`...     
.........`yNNy`...................  ..  ..   ..`yNNNNNNNWWWWNW?`.     
.......`xyyxyyy?`..`yWWWWWWy`..........`yNWWWWNNNNNNNNNNWWWWNWx!.     
.......`yWyxxxxyNNNNNNWWWWWWWWWWWWWWWWWWWWWWWWWWWWNNNNNNNNWWNWx!.     
.....`xyyyyyxxxyNNNNWWNNNNWWWWWWWWWWWWWWWWWWWyyyyWWWWNNNNNWWNW?`.     
.....`yWxxyyWWWNNNNNWWNNNNNNNNWWWWyyyyyyWWyyyxxxyyyyyyWNNNNNNW?`.     
...`xWWyxxxxyyWWxxWNNNWyyyWNNNNNWWWWyyxxyyyyyyxxxyyyxxxyWNNNNWx!.     
...`yWyyyyWWyx?!`!yNNWy???xyWNNNNNNWyyxxxyyyyyxxxxxxxxxxxyNNNW?`.     
...`yWyyyyyWWy`..!yNNWyx?!!?xyWNNNWyyxxxxxxxxxxxxxxxxxxxyWNNNW?!.     
...`yWyyyyWWWx`..`?yWNWyx?!!!xyWNNWyxxxxxxxxxxxxxxxxxxyWNNWWNW?!.     
...`yWyyyyWWWx!....!yNNNWyx?xyWNNNWyyxxxxxxxxxxxxxxxxyNNNWxxWW?`.     
...`yNWyyyyWWx`....`?yNNNNWWNNNWWWyyxxyyyyyxxxxxxxxxxxxyWW??yyy?`     
...`yNWyyyWWWy?`....`?WNNNNNNWyxxxxxyWNNNNNWyxxx????xx??yy?!xyWWWy?!..
  .`yWyyyyxyWWWy?!!?WNNNWWWyyxxxxyWWWNNNNNNNNWx!...`!xWNNW?!!?xyx?!`. 
...`xyyxxyWWWWWWNNNNNNWyyyyxxxxx?xyyyyxyWNNNNWx!.....`?xyx??xWWWWy?!..
   ..`yNNWWWWWNNNWWWWWyyyxxxxxxx!!!!!!.`!xWNNWx!`..........`xx`.......
.....`yWWWWWNNWWWx!?WNWWWyyyyyxx!``...`!xWWWyx`..`?xx?`....`??`       
   ....!yNNNWWWyyyxyWNWyyxxyyWNWyxx?!``!?xxxx!`.`!yNWy`..`!xyx`.. ....
 .... .`yWyyWWWyyxxxxxxxxxxx!````............... ...````?xyx`..   ..  
 ....`yWWyyyyyyxxxxyyyyxxxxxxyyWWWWy?!. .....  ......`!xyx`.. .. .....
.....!yNWyxxxxxxxxyyWWyxxxxxyyyWWWx?`..........`!xyyyyyx`......       
     `yNWyxxxxxxyyyyWyyyyyxxxxxxyyxx??????????xxWWWy?`.....  ....     
 ....!yNWyyyxxxxxyyyyyxyyyyxyyyyxxxxxxxxx?!?xyyyyyyyy?`......         
 ..`xWWWWWyyxxxxxyyyyxxxyyyyyyWWWWyyyyx?```!xyyyyyyyWWy?`. ......     
...`yNWWWWWWyyxxx?!!!!!!?xxxyyyyWWyWWWx?``?xyy!`!!!!!xWWy?` ..        
 ..`yWWyyyyyyxxxx?`..  .`!??xxyyyyyyWNWyyyWWWx`..   ..!xWy`......     
...`xyx?xyxxxxx?!`..`!???xyyx???????xxyWWWyxx??!....``.!yy`           
 ..`??``!!!?????????yyyyyyyx???!``````!?xx!`!?Wy?``!!!.`yy`......     
  .`??`..  ..!xWNNNNNy!....`xyx!`..  ..!xyyx!!xyWyyy?!!?x?`  ....     
 ..`??`....`!?yWWWyyyx`....`?xyy?``.````!?yx`..`xyWWyyWx`......       
...`?x!```?xxx`..  ......  ..`xxx??????!??x?`..... .....  ....        
 ..`!xyWWWy?`......    ......`?xyyyx????x?`    ............  ....     
   ....       ..   ........  ..`?yWWWWy?`...........  .. ......       
..... .........  .....   ......       ..   ....  ................
```