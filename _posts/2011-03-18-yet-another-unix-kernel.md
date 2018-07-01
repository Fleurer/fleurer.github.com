---
layout: post
title: "另一个unix-like内核, Fleurix"
---

该是在去年五月份，照着网上的教程写了几行helloworld，简简单单的int 13h放在virtualbox里打印出来一段红色的“screw you guys all fucked up~”。随后从这段不得体的汇编开始，慢慢地文件系统、内存管理、输入输出、进程管理等等初具轮廓，到现在一个相对完整的内核，不觉已过了九个月。时间就是个见证成长的东西 :)

<a href="https://github.com/fleurer/fleurix">https://github.com/fleurer/fleurix</a>

37个系统调用，七千行C，二百多行汇编。没有管道，没有swap，也不是基于POSIX，各种特性是能删即删，能简即简。不过也算完成了它的设计目标，那就是跑起来。Fleurix已经有了：

<ul>
<li> minix v1的文件系统。原理简单，而且可以利用linux下的mkfs.minix，fsck.minix等工具。
<li> fork()/exec()/exit()等等。a.out的可执行格式，实现了写时复制与请求调页。
<li> 信号。
<li> 一个纯分页的内存管理系统，每个进程4gb的地址空间，共享128mb的内核地址空间。至少比Linux0.11中的段页式内存管理方式更加灵活。
<li> 一个简单的kmalloc()(可惜没大用上)。
<li> 一个简单的终端。
</ul>

<img src="http://i.min.us/im2snS.jpg"></img>
<img src="http://i.min.us/ikheqK.jpg"></img>

硬伤就是，没有硬盘分区，内存也写死了128mb，恐怕无法在真机上运行。

<hr />

编译环境: ubuntu
工具: rake, binutils(gcc, ld), nasm, bochs, mkfs.minix

<pre code="bash">
git clone git@github.com:Fleurer/fleurix.git
cd fleurix
rake
</pre>
