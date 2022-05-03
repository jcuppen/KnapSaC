#!/bin/sh

print_header() {
	echo "\n###########\n$1\n###########\n"
}

touch man.txt

print_header "Main" > man.txt
target/debug/knapsac --help >> man.txt
print_header "Initialize" >> man.txt
target/debug/knapsac initialize --help >> man.txt
print_header "Add" >> man.txt
target/debug/knapsac add --help >> man.txt
print_header "Remove" >> man.txt
target/debug/knapsac remove --help >> man.txt
print_header "Download" >> man.txt
target/debug/knapsac download --help >> man.txt
