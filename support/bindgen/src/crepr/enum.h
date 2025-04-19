typedef enum
{
	A = (1<<0),
	B = (1<<1),
	C = (1<<2),
	D = (2<<2),
} MyEnum;

// clang -target x86_64-pc-win32-msvc -Xclang -ast-dump -fsyntax-only enum.h
