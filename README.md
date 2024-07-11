### Rust ShellCode Executer
このプログラムはウイルス解析/検知回避などの研究目的で作成しました。<BR>
他人に害を与えるために使用しないでください。
<BR>
##### RustでWinAPIを使用しShellCodeを実行するプログラム集
- winapi_mal_mk2 <BR>
  ShellCodeを文字列に変換し、静的解析を回避するために作成したプログラム<BR>
  WinAPIを使用し自己のメモリ空間上でShellCodeを実行する。<BR>
  2024/06/28時点でWindows Defender回避可能<BR>

- Process_Hollowing_CS<BR>
  CobaltStrikeのVBAペイロードを格納して実行するWINAPI<BR>
  実行時にはプロセスホローイングを行い、自己を隠ぺいする。<BR>
