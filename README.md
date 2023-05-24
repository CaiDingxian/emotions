# Emotions

一个PC端动态表情包软件，基于GTK4-RS 

![image](https://github.com/CaiDingxian/emotions/assets/37413956/6db492ef-450a-4830-801f-4342f0620604)


## 基本功能特点

1.不需要安装，打开能直接使用

2.单击可以复制动图，可以粘贴到 微信 / QQ / 钉钉 等任意聊天工具，不会变成文件或静态图。

3.图片自动加载到软件目录下的emo_res文件中，可以任意复制或删除。

4.图片来自网络，不做过度限制与过滤，保留聊天原始的乐趣

## 规划中的功能

1.表情包分类搜索

2.斗图系列表情包，真正体现“包”的概念

3.表情包收藏

4.表情工坊，动图生成功能

5.支持Linux桌面端和Mac os

6.快捷键输入并发图到聊天工具

7.表情包云同步

## 为什么要做这款软件

1.学习Rust

2.被各种输入法表情包折磨（痛苦面具）：

（1）某度输入法的表情包有恶性bug，运行会卡炸。

（2）某Q输入法连自家聊天软件都兼容不好，狂搓几下才能发出，另有表情包云同步失败的问题

（3）某狗输入法的广告太多

（4）这些大厂输入法图片过滤太严格，好多好玩的图找不到了


3.可以加入一些比较骚的功能


## 环境

基于MinGW和Gtk4-rs，环境配置可见gtk4-rs官方文档

理论上支持跨多端。当前只在windows上测试过。（Linux理论上能用）

目前Gtk原生剪贴板对windows的gif复制无效，所以代码中的剪贴板使用了Windows-rs的api

在windows程序exe的目录下需要放置以下dll

```ps
/msys64/mingw64/bin/libgdk_pixbuf-2.0-0.dll            
/msys64/mingw64/bin/libcairo-2.dll                     
/msys64/mingw64/bin/libglib-2.0-0.dll                  
/msys64/mingw64/bin/libgio-2.0-0.dll                   
/msys64/mingw64/bin/libgraphene-1.0-0.dll              
/msys64/mingw64/bin/libgobject-2.0-0.dll               
/msys64/mingw64/bin/libgtk-4-1.dll                     
/msys64/mingw64/bin/libpango-1.0-0.dll                 
/msys64/mingw64/bin/libgmodule-2.0-0.dll               
/msys64/mingw64/bin/libintl-8.dll                      
/msys64/mingw64/bin/libgcc_s_seh-1.dll                 
/msys64/mingw64/bin/libstdc++-6.dll                    
/msys64/mingw64/bin/libfontconfig-1.dll                
/msys64/mingw64/bin/libfreetype-6.dll                  
/msys64/mingw64/bin/libpixman-1-0.dll                  
/msys64/mingw64/bin/libpng16-16.dll                    
/msys64/mingw64/bin/zlib1.dll                          
/msys64/mingw64/bin/libpcre2-8-0.dll                   
/msys64/mingw64/bin/libffi-8.dll                       
/msys64/mingw64/bin/libfribidi-0.dll                   
/msys64/mingw64/bin/libharfbuzz-0.dll                  
/msys64/mingw64/bin/libthai-0.dll                      
/msys64/mingw64/bin/libiconv-2.dll                     
/msys64/mingw64/bin/libwinpthread-1.dll                
/msys64/mingw64/bin/libwinpthread-1.dll                
/msys64/mingw64/bin/libexpat-1.dll                     
/msys64/mingw64/bin/libbz2-1.dll                       
/msys64/mingw64/bin/libbrotlidec.dll                   
/msys64/mingw64/bin/libgraphite2.dll                   
/msys64/mingw64/bin/libdatrie-1.dll                    
/msys64/mingw64/bin/libcairo-gobject-2.dll             
/msys64/mingw64/bin/libcairo-script-interpreter-2.dll  
/msys64/mingw64/bin/libepoxy-0.dll                     
/msys64/mingw64/bin/libjpeg-8.dll                      
/msys64/mingw64/bin/libpangowin32-1.0-0.dll            
/msys64/mingw64/bin/libpangocairo-1.0-0.dll            
/msys64/mingw64/bin/libtiff-6.dll                      
/msys64/mingw64/bin/libbrotlicommon.dll                
/msys64/mingw64/bin/liblzo2-2.dll                      
/msys64/mingw64/bin/libpangoft2-1.0-0.dll              
/msys64/mingw64/bin/libdeflate.dll                     
/msys64/mingw64/bin/libjbig-0.dll                      
/msys64/mingw64/bin/libLerc.dll                        
/msys64/mingw64/bin/liblzma-5.dll                      
/msys64/mingw64/bin/libwebp-7.dll                      
/msys64/mingw64/bin/libzstd.dll                        
/msys64/mingw64/bin/libsharpyuv-0.dll                  
```
