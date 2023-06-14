; get the name of every element
push "Elements"
query
iload 0 {
	push "Name"
	info
}
push 0 ; function register
map

; convert them to numbers
iload 0 {tonum}
push 0 ; function register
map

; keep only valid numbers
iload 0 {
	push nil
	!=
}
push 0 ; function register
filter

; get the sum
iload 0 {+}
push 0 ; function register
push 0 ; initial number to add the values to
reduce

; automatically return sum as it's left on the stack