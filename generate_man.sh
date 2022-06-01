#!/bin/sh

print_header() {
	echo -e "\n###########\n$1\n###########\n"
}

touch man.txt

print_header "Main" > man.txt
target/debug/knapsac --help >> man.txt
print_header "Initialize" >> man.txt
target/debug/knapsac initialize --help >> man.txt
print_header "Add" >> man.txt
target/debug/knapsac add_dependency --help >> man.txt
print_header "Remove" >> man.txt
target/debug/knapsac remove --help >> man.txt
#print_header "Download" >> man.txt
#target/debug/knapsac download --help >> man.txt
#print_header "Add Dependency" >> man.txt
#target/debug/knapsac dependencies-dependency --help >> man.txt
#print_header "Remove Dependency" >> man.txt
#target/debug/knapsac remove-dependency --help >> man.txt
#print_header "Add Module" >> man.txt
#target/debug/knapsac dependencies-module --help >> man.txt
#print_header "Remove Module" >> man.txt
#target/debug/knapsac remove-module --help >> man.txt
