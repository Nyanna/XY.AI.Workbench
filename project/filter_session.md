Der Control Handler soll sie Möglichkeit bieten offene Control Requests anhand der Session zu Filtern über den HTTP ENdpunkt zu Filtern.

`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/handler.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/__init__.py`

Der Control Client soll die Möglichkeit nutzen und anhand der Session filtern in der er sich beefindet.

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java`

Das soll das aktuelle Problem lösen, das Control Request kreuz und quer in den Sessions landen und es soll die Parallele Ausführbarkeit von Sessions verbessern.