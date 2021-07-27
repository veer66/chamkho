# Chamkho
Khmer/Lao/Myanmar/Thai word segmentation library in Rust

## Usage

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
