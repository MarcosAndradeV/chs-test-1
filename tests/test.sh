#! /usr/sbin/zsh



for filename in ./*.chs; do
    echo $filename > ./output/$filename-output.txt
    cargo run -- $filename > ./output/$filename-output.txt
done