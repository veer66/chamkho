# Chamkho
Khmer, Lao, Myanmar, and Thai word segmentation/breaking library and command line

## Algorithm

https://github.com/veer66/wordcut-engine

## C-ABI Library

https://github.com/veer66/wordcutw

## Usage

### Binary tarball

#### On Google Colab

```
!wget -q https://github.com/veer66/chamkho/releases/download/1.4.0/chamkho-1.4.0-linux-x64-musl.tar.gz -O - | tar -xzf -
with open('input.txt', 'w') as f:
  f.write(u'กาเตเภไก่')
!cd chamkho-1.4.0-linux-x64-musl; ./wordcut < ../input.txt; cd ..
```

#### On GNU/Linux
```
wget -q https://github.com/veer66/chamkho/releases/download/1.4.0/chamkho-1.4.0-linux-x64-musl.tar.gz -O - | tar -xzf - 
cd chamkho-1.4.0-linux-x64-musl
echo กากากา | ./wordcut
cd ..
```

#### On Windows Powershell

```
PS C:\ex1> $OutputEncoding = [console]::InputEncoding = [console]::OutputEncoding = New-Object System.Text.UTF8Encoding
PS C:\ex1> Invoke-WebRequest -uri https://github.com/veer66/chamkho/releases/download/1.1.0/chamkho-1.1.0-windows-amd64.zip -OutFile chamkho.zip
PS C:\ex1> Expand-Archive -Path .\chamkho.zip -DestinationPath .
PS C:\ex1> cd .\chamkho-1.1.0-windows-amd64\
PS C:\ex1\chamkho-1.1.0-windows-amd64> echo ฉันง่วงมาก | .\wordcut.
exe
ฉัน|ง่วง|มาก
```

### As command line

    echo "กากกา" | wordcut 

#### External dictionary

    echo "กากกา" | wordcut -d <path to dictionary>

#### Specific language

```Bash
echo "ພາສາລາວມີ" | wordcut -l lao
```

```Bash
echo នៅក្នុងការប្រកបរបរអាជីវកម្ | wordcut -l khmer
```

```Bash
echo မြန်မာမှာ | wordcut -l myanmar
```

#### WebAssembly

```Bash
cargo build --target=wasm32-wasi
wasmtime --dir=$(pwd)/data target/wasm32-wasi/debug/wordcut.wasm
```

## Benchmark

### Setup

* Computer: Hetzner's CX11
* Rustc: rustc 1.53.0 (53cb7b09b 2021-06-17)
* OS: Linux exper1 5.12.15-300.fc34.x86_64 #1 SMP Wed Jul 7 19:46:50 UTC 2021 x86_64 x86_64 x86_64 GNU/Linux
* Script:

```Bash
#!/bin/bash
set -x
INPUT=thwik-head1m.txt
#INPUT=x.txt
for i in {1..10}
do
  { time nlpo3 segment < $INPUT > o3 ; } 2>> bench_o3.txt
  { time wordcut < $INPUT > wc.txt ; } 2>> bench_wc.txt
done
```

* nlpo3 version: 1.1.2
* nlpo3-cli version: 0.0.1
* chamkho version: 0.5.0
* dataset: https://file.veer66.rocks/langbench/thwik-head1m.txt

### Result

#### nlpo3

```
[root@exper1 ~]# grep real bench_o3.txt 
real    3m26.884s
real    3m15.001s
real    3m12.829s
real    3m11.998s
real    3m12.399s
real    3m13.829s
real    3m14.506s
real    3m9.198s
real    3m6.749s
real    3m8.729s
```

#### chamko

```
[root@exper1 ~]# grep real bench_wc.txt 
real    1m41.611s
real    1m40.262s
real    1m40.488s
real    1m40.765s
real    1m39.385s
real    1m41.002s
real    1m38.292s
real    1m35.906s
real    1m40.263s
real    1m36.523s
```

#### Average
* nlpo3: 193.21s
* chamkho:  99.44s


## Benchmark 2

Chamkho and Newmm on Mac mini M1

### Setup

* Computer: Scaleway's Mac mini M1
* Rustc: rustc 1.54.0 (a178d0322 2021-07-26)
* Python: Python 3.8.2
* OS: Darwin 506124d8-4acf-4595-9d46-8ca4b44b8110 20.6.0 Darwin Kernel Version 20.6.0: Wed Jun 23 00:26:27 PDT 2021; root:xnu-7195.141.2~5/RELEASE_ARM64_T8101 arm64
* Script:

```Bash
#!/bin/bash

set -x

INPUT=thwik-head1m.txt

for i in {1..10}
do
  { time python3 newmm.py < $INPUT > newmm.out ; } 2>> bench_newmm.txt
  { time wordcut < $INPUT > cham.out ; } 2>> bench_chamkho.txt
done
```
* PyThaiNLP: 2.3.1
* chamkho version: 1.0.2
* dataset: https://file.veer66.rocks/langbench/thwik-head1m.txt

### Result

#### newmm

```
real    7m40.693s
real    7m40.623s
real    7m41.623s
real    7m40.438s
real    7m41.363s
real    7m39.108s
real    7m39.486s
real    7m39.946s
real    7m39.960s
real    7m40.279s
```

#### chamko

```
real    1m2.110s
real    1m2.200s
real    1m1.954s
real    1m1.823s
real    1m1.836s
real    1m1.864s
real    1m1.638s
real    1m1.641s
real    1m1.688s
real    1m1.923s
```

#### Average
* newmm

```
$ grep real bench_newmm.txt | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}'
460.3519
```

* chamkho: 

```
$ grep real bench_chamkho.txt  | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}'
61.8677
```

#### Performance ratio

7.4

## Benchmark 3

Chamkho and Newmm on Xeon

### Setup

* Computer: Hetzner's CX11
* Rustc: rustc 1.54.0 (a178d0322 2021-07-26)
* Python: Python 3.9.6
* OS: Linux fedora-2gb-hel1-1 5.12.15-300.fc34.x86_64 #1 SMP Wed Jul 7 19:46:50 UTC 2021 x86_64 x86_64 x86_64 GNU/Linux
* Script:

```Bash
#!/bin/bash
set -x

INPUT=thwik-head1m.txt

for i in {1..10}
do
  { time python newmm.py < $INPUT > newmm.out ; } 2>> bench_newmm.txt
  { time wordcut < $INPUT > cham.out ; } 2>> bench_chamkho.txt
done
```

* PyThaiNLP: 2.3.1
* chamkho version: 1.0.2
* dataset: https://file.veer66.rocks/langbench/thwik-head1m.txt

### Result

#### newmm

```
# grep real bench_newmm.txt 
real    17m15.608s
real    17m14.038s
real    17m7.864s
real    17m17.329s
real    17m5.501s
real    17m10.841s
real    17m16.348s
real    17m19.813s
real    17m28.796s
real    17m26.056s
```

#### chamko

```
# grep real bench_chamkho.txt 
real    1m46.157s
real    1m47.785s
real    1m47.173s
real    1m45.656s
real    1m45.554s
real    1m46.612s
real    1m48.991s
real    1m49.656s
real    1m47.677s
real    1m47.876s
```

#### Average
* newmm

```
# grep real bench_newmm.txt | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}'
1036.2194000000002
```

* chamkho: 

```
$ # grep real bench_chamkho.txt  | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}'
107.31370000000001
```

#### Performance ratio

9.65



## Benchmark 4

Chamkho and Nlpo3 on Mac mini M1

### Setup

* Computer: Scaleway's Mac mini M1
* Rustc: rustc 1.54.0 (a178d0322 2021-07-26)
* OS: Darwin 506124d8-4acf-4595-9d46-8ca4b44b8110 20.6.0 Darwin Kernel Version 20.6.0: Wed Jun 23 00:26:27 PDT 2021; root:xnu-7195.141.2~5/RELEASE_ARM64_T8101 arm64
* Script:

```Bash
#!/bin/bash

set -x

INPUT=thwik-head1m.txt

for i in {1..10}
do
  { time wordcut < $INPUT > newmm.out ; } 2>> bench_chamkho.txt
  { time nlpo3 segment < $INPUT > cham.out ; } 2>> bench_o3.txt
done
```

* nlpo3-cli version: 0.0.1
* chamkho version: 1.0.2
* dataset: https://file.veer66.rocks/langbench/thwik-head1m.txt

### Result

#### nlpo3

```
% grep real bench_o3.txt       
real    2m7.639s
real    2m7.024s
real    2m6.296s
real    2m7.731s
real    2m7.873s
real    2m7.028s
real    2m6.411s
real    2m6.974s
real    2m7.746s
real    2m6.955s
```

#### chamko

```
% grep real bench_chamkho.txt 
real    1m0.237s
real    1m1.799s
real    1m1.752s
real    1m1.373s
real    1m1.128s
real    1m1.870s
real    1m1.878s
real    1m1.709s
real    1m1.690s
real    1m1.030s
```

#### Average
* nlpo3

```
% grep real bench_o3.txt | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}'
127.1677
```

* chamkho: 

```
% grep real bench_chamkho.txt | ruby -lane 'BEGIN { all = 0.0; cnt = 0 }; cols = $F[1].split(/[ms]/).map {|x| x.to_f }; v = cols[0]*60 + cols[1]; all += v; cnt += 1; END { p all/cnt}' 
61.44659999999999
```
    
#### Performance ratio

2.07
