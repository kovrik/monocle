Monocle
---

Java Class File Disassembler in Rust

Usage:
---

Pass path to .class file as the first argument.

Things implemented so far:
- magic
- minor_version
- major_version
- constant_pool_count
- constant_pool
- access_flags
- this_class
- super_class
- interfaces_count
- interfaces
- fields_count
- methods_count
- methods\*
- attributes_count

(\* partially)

TODO:
---
- implement: attributes, methods, fields
- fix precision loss when reading very small Float/Double values
- format numbers
- compiled_from
- print bytecode
- clean up, optimize the code, make it more idiomatic

Example:
---

``` sh
> monocle test/Test.class
___________________________________________
test/Test.class
Classfile test/Test.class
  Last modified 17/10/2016; size: 1128 bytes
  MD5 checksum 972edd2712832870499312b2ebd4619a
  Compiled from "NOT IMPLEMENTED"
  minor version: 0
  major version: 52
Constant pool count: 86
Constant pool:
	#1 = Methodref		#6.#71     // java/lang/Object.<init>.()V
	#2 = String		#72			 // Hello
	#3 = Fieldref		#73.#74			 // java/lang/System.out.Ljava/io/PrintStream;
	#4 = Methodref		#75.#76			 // java/io/PrintStream.println.(Ljava/lang/String;)V
	#5 = Class		#77			 // Test
	#6 = Class		#78			 // java/lang/Object
	#7 = Class		#79			 // java/io/Serializable
	#8 = Class		#80			 // java/lang/Runnable
	#9 = Utf8		serialVersionUID			
	#10 = Utf8		J			
	#11 = Utf8		ConstantValue			
	#12 = Long		1			
	#14 = Utf8		intv			
	#15 = Utf8		I			
	#16 = Integer		38944985			
	#17 = Utf8		int_max			
	#18 = Integer		2147483647			
	#19 = Utf8		int_min			
	#20 = Integer		-2147483648			
	#21 = Utf8		l			
	#22 = Long		1234			
	#24 = Utf8		min			
	#25 = Long		9223372036854775807			
	#27 = Utf8		max			
	#28 = Long		-9223372036854775808			
	#30 = Utf8		d			
	#31 = Utf8		D			
	#32 = Double		3452.776655			
	#34 = Utf8		nand			
	#35 = Double		NaNd			
	#37 = Utf8		dmax			
	#38 = Double		179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000			
	#40 = Utf8		dmin			
	#41 = Double		0			
	#43 = Utf8		dinfp			
	#44 = Double		Infinityf			
	#46 = Utf8		dinfm			
	#47 = Double		-Infinityf			
	#49 = Utf8		f1			
	#50 = Utf8		F			
	#51 = Float		-3345.345			
	#52 = Utf8		nanf			
	#53 = Float		NaNf			
	#54 = Utf8		fmax			
	#55 = Float		340282350000000000000000000000000000000			
	#56 = Utf8		fmin			
	#57 = Float		0			
	#58 = Utf8		finfp			
	#59 = Float		Infinityf			
	#60 = Utf8		finfm			
	#61 = Float		-Infinityf			
	#62 = Utf8		<init>			
	#63 = Utf8		()V			
	#64 = Utf8		Code			
	#65 = Utf8		LineNumberTable			
	#66 = Utf8		main			
	#67 = Utf8		([Ljava/lang/String;)V			
	#68 = Utf8		run			
	#69 = Utf8		SourceFile			
	#70 = Utf8		Test.java			
	#71 = NameAndType	#62.#63			 // <init>.()V
	#72 = Utf8		Hello			
	#73 = Class		#81			 // java/lang/System
	#74 = NameAndType	#82.#83			 // out.Ljava/io/PrintStream;
	#75 = Class		#84			 // java/io/PrintStream
	#76 = NameAndType	#85.#86			 // println.(Ljava/lang/String;)V
	#77 = Utf8		Test			
	#78 = Utf8		java/lang/Object			
	#79 = Utf8		java/io/Serializable			
	#80 = Utf8		java/lang/Runnable			
	#81 = Utf8		java/lang/System			
	#82 = Utf8		out			
	#83 = Utf8		Ljava/io/PrintStream;			
	#84 = Utf8		java/io/PrintStream			
	#85 = Utf8		println			
	#86 = Utf8		(Ljava/lang/String;)V			

	flags: ACC_PUBLIC, ACC_SUPER

This class: Test
Super class: java/lang/Object

Interfaces count: 2
	Interface: java/io/Serializable
	Interface: java/lang/Runnable
Fields count: 19
 
Bytes:
00000000:  cafe babe 0000 0034 0057 0a00 0600 4708
00000010:  0048 0900 4900 4a0a 004b 004c 0700 4d07
00000020:  004e 0700 4f07 0050 0100 1073 6572 6961
00000030:  6c56 6572 7369 6f6e 5549 4401 0001 4a01
00000040:  000d 436f 6e73 7461 6e74 5661 6c75 6505
00000050:  0000 0000 0000 0001 0100 0469 6e74 7601
00000060:  0001 4903 0252 40d9 0100 0769 6e74 5f6d
00000070:  6178 037f ffff ff01 0007 696e 745f 6d69
00000080:  6e03 8000 0000 0100 016c 0500 0000 0000
00000090:  0004 d201 0003 6d69 6e05 7fff ffff ffff
000000a0:  ffff 0100 036d 6178 0580 0000 0000 0000
000000b0:  0001 0001 6401 0001 4406 40aa f98d a5b9
000000c0:  628d 0100 046e 616e 6406 7ff8 0000 0000
000000d0:  0000 0100 0464 6d61 7806 7fef ffff ffff
000000e0:  ffff 0100 0464 6d69 6e06 0000 0000 0000
000000f0:  0001 0100 0564 696e 6670 067f f000 0000
00000100:  0000 0001 0005 6469 6e66 6d06 fff0 0000
00000110:  0000 0000 0100 0266 3101 0001 4604 c551
00000120:  1585 0100 046e 616e 6604 7fc0 0000 0100
00000130:  0466 6d61 7804 7f7f ffff 0100 0466 6d69
00000140:  6e04 0000 0001 0100 0566 696e 6670 047f
00000150:  8000 0001 0005 6669 6e66 6d04 ff80 0000
00000160:  0100 063c 696e 6974 3e01 0003 2829 5601
00000170:  0004 436f 6465 0100 0f4c 696e 654e 756d
00000180:  6265 7254 6162 6c65 0100 046d 6169 6e01
00000190:  0016 285b 4c6a 6176 612f 6c61 6e67 2f53
000001a0:  7472 696e 673b 2956 0100 0372 756e 0100
000001b0:  0a53 6f75 7263 6546 696c 6501 0009 5465
000001c0:  7374 2e6a 6176 610c 003e 003f 0100 0548
000001d0:  656c 6c6f 0700 510c 0052 0053 0700 540c
000001e0:  0055 0056 0100 0454 6573 7401 0010 6a61
000001f0:  7661 2f6c 616e 672f 4f62 6a65 6374 0100
00000200:  146a 6176 612f 696f 2f53 6572 6961 6c69
00000210:  7a61 626c 6501 0012 6a61 7661 2f6c 616e
00000220:  672f 5275 6e6e 6162 6c65 0100 106a 6176
00000230:  612f 6c61 6e67 2f53 7973 7465 6d01 0003
00000240:  6f75 7401 0015 4c6a 6176 612f 696f 2f50
00000250:  7269 6e74 5374 7265 616d 3b01 0013 6a61
00000260:  7661 2f69 6f2f 5072 696e 7453 7472 6561
00000270:  6d01 0007 7072 696e 746c 6e01 0015 284c
00000280:  6a61 7661 2f6c 616e 672f 5374 7269 6e67
00000290:  3b29 5600 2100 0500 0600 0200 0700 0800
000002a0:  1300 1a00 0900 0a00 0100 0b00 0000 0200
000002b0:  0c00 1a00 0e00 0f00 0100 0b00 0000 0200
000002c0:  1000 1a00 1100 0f00 0100 0b00 0000 0200
000002d0:  1200 1a00 1300 0f00 0100 0b00 0000 0200
000002e0:  1400 1a00 1500 0a00 0100 0b00 0000 0200
000002f0:  1600 1a00 1800 0a00 0100 0b00 0000 0200
00000300:  1900 1a00 1b00 0a00 0100 0b00 0000 0200
00000310:  1c00 1a00 1e00 1f00 0100 0b00 0000 0200
00000320:  2000 1a00 2200 1f00 0100 0b00 0000 0200
00000330:  2300 1a00 2500 1f00 0100 0b00 0000 0200
00000340:  2600 1a00 2800 1f00 0100 0b00 0000 0200
00000350:  2900 1a00 2b00 1f00 0100 0b00 0000 0200
00000360:  2c00 1a00 2e00 1f00 0100 0b00 0000 0200
00000370:  2f00 1a00 3100 3200 0100 0b00 0000 0200
00000380:  3300 1a00 3400 3200 0100 0b00 0000 0200
00000390:  3500 1a00 3600 3200 0100 0b00 0000 0200
000003a0:  3700 1a00 3800 3200 0100 0b00 0000 0200
000003b0:  3900 1a00 3a00 3200 0100 0b00 0000 0200
000003c0:  3b00 1a00 3c00 3200 0100 0b00 0000 0200
000003d0:  3d00 0300 0100 3e00 3f00 0100 4000 0000
000003e0:  1d00 0100 0100 0000 052a b700 01b1 0000
000003f0:  0001 0041 0000 0006 0001 0000 0003 0009
00000400:  0042 0043 0001 0040 0000 002b 0002 0002
00000410:  0000 000b 1202 4cb2 0003 2bb6 0004 b100
00000420:  0000 0100 4100 0000 0e00 0300 0000 2000
00000430:  0300 2100 0a00 2200 0100 4400 3f00 0100
00000440:  4000 0000 1900 0000 0100 0000 01b1 0000
00000450:  0001 0041 0000 0006 0001 0000 0025 0001
00000460:  0045 0000 0002 0046
``` java
