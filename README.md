# Embedded Systems Praktikum 02

Dieses Repository enthält die Lösung für das Embedded Systems Praktikum 02 in Rust. Dazu verwenden wir Embassy als RTOS auf einem STM32F407VG-Discovery Board.

Das Projekt ist ein Spiel, welches durch drücken des USER-Buttons gestartet wird. Nach Ablauf eines random Timers leuchtet eine der 4 LEDs auf. Der Spieler muss das Board dann innerhalb von 2 Sekunden in diese Richtung neigen. Wenn er es schafft, leuchtet die Status LED grün, andernfalls rot. Das Spiel kann beliebig oft wiederholt werden.
