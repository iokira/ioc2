#!/bin/bash

IOC=./target/release/ioc2
MAINC=./source/main.c
MAINS=./source/main.s
MAIN=./source/main

mkdir -p ./source

assert() {
    expected="$1"
    input="$2"

    echo $input > $MAINC
    $IOC $MAINC $MAINS
    gcc $MAINS -o $MAIN
    $MAIN
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual" >&2
        exit 1
    fi
}

# ダブルクオーテーションの中でも*の後ろに空白文字があるとメタ文字と解釈されてファイル一覧に展開されるから注意

assert 0 "0;"
assert 42 "42;"
assert 41 " 12 + 34 -5 ;"
assert 47 "5 + 6 *7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"
assert 10 "-10+20;"
assert 1 "1==1;"
assert 1 "2 *3!=3-1;"
assert 0 "32/(1==1)<2;"
assert 2 "a=1+1;a;"
assert 14 "a = 3; b = 5*6 - 8; a + b / 2;"
assert 15 "row = 3; column=5; row*column;"
assert 14 "a = 3; b = 5*6 - 8; return a + b / 2;"
assert 5 "return 5; return8;"
assert 0 "if(1>0)return 0;return 1;"
assert 5 "if(3 > 2)return 5; else return 4;"
assert 4 "i = 1; while(i < 3) i = i*2;"
assert 10 "for(a = 0; a < 10; a = a + 1)a + 1;"
assert 121 "a=1;while(a<100)for(i = 0; i < 10; i = i + 1)if(i<5)a=a+1;else a=a+i;a;"
assert 10 "{{{{{{{{{{0;1;2;3;10;}}}}}}}}}}"
assert 55 "
num = 0;
for (i = 1; i <= 10; i = i + 1) {
    num = num + i;
}
return num;
"
assert 29 "
num = 0;
for (i = 1; i <= 10; i = i + 1) {
    num = num + 1;
    if (i == 2) {
        num = num + 1;
    } else {
        num = num + 2;
    }
    a = 0;
    b = 1;
    c = 2;
}
return num;
"

echo OK

